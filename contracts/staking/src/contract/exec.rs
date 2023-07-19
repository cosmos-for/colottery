use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, DepsMut, DistributionMsg, Env, MessageInfo, QuerierWrapper,
    Response, StakingMsg, StdError, StdResult, Uint128, WasmMsg,
};
use cw20_base::{
    allowances::{
        execute_burn_from, execute_decrease_allowance, execute_increase_allowance,
        execute_send_from, execute_transfer_from,
    },
    contract::{execute_burn, execute_mint, execute_send, execute_transfer},
};

use crate::{
    msg::ExecuteMsg,
    state::{Supply, CLAIMS, INVESTMENT, TOTAL_SUPPLY},
    ContractError,
};

use super::FALLBACK_RATIO;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Bond {} => bond(deps, env, info),
        Unbond { amount } => unbound(deps, env, info, amount),
        Claim {} => claim(deps, env, info),
        Reinvest {} => reinvest(deps, env, info),
        BondAllTokens {} => bond_all_tokens(deps, env, info),

        // cw20-base implementation
        Transfer { recipient, amount } => Ok(execute_transfer(deps, env, info, recipient, amount)?),
        Burn { amount } => Ok(execute_burn(deps, env, info, amount)?),
        Send {
            contract,
            amount,
            msg,
        } => Ok(execute_send(deps, env, info, contract, amount, msg)?),
        IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        TransferFrom {
            owner,
            recipient,
            amount,
        } => Ok(execute_transfer_from(
            deps, env, info, owner, recipient, amount,
        )?),
        BurnFrom { owner, amount } => Ok(execute_burn_from(deps, env, info, owner, amount)?),
        SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => Ok(execute_send_from(
            deps, env, info, owner, contract, amount, msg,
        )?),
    }
}

pub fn bond(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // ensure the denom
    let invest = INVESTMENT.load(deps.storage)?;
    // payment finds the proper coin, otherwise it returns an error
    let payment = info
        .funds
        .iter()
        .find(|c| c.denom == invest.bond_denom)
        .ok_or_else(|| ContractError::EmptyBalance {
            denom: invest.bond_denom.clone(),
        })?;

    // bonded is the total number of tokens we have delegated from this address
    let bonded = get_bounded(&deps.querier, &env.contract.address)?;
    println!("Current bonded amount is: {} in bond function", bonded);

    // calculate to_mint and update total supply
    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;
    println!("Current supply amount is: {:?} in bond function", supply);

    assert_bonds(&supply, bonded)?;

    let to_mint = if supply.issued.is_zero() || bonded.is_zero() {
        FALLBACK_RATIO * payment.amount
    } else {
        payment.amount.multiply_ratio(supply.issued, bonded)
    };

    supply.bonded = bonded + payment.amount;
    supply.issued += to_mint;

    println!(
        "After bounded, total supply is: {:?} in bond function",
        supply
    );

    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    let mint_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    // self mint the derivation token (denom is in the instantiate message) for sender (delegator)
    execute_mint(deps, env, mint_info, info.sender.to_string(), to_mint)?;

    // delegate them to the validator
    let resp = Response::new()
        .add_message(StakingMsg::Delegate {
            validator: invest.validator,
            amount: payment.clone(),
        })
        .add_attribute("action", "bond")
        .add_attribute("from", info.sender)
        .add_attribute("bonded", payment.amount.to_string())
        .add_attribute("minted", to_mint);

    Ok(resp)
}

pub fn unbound(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    println!("Entering unbonding function ...");

    let invest = INVESTMENT.load(deps.storage)?;

    if amount < invest.min_withdrawal {
        return Err(ContractError::UnbondTooSmall {
            min_bonded: invest.min_withdrawal,
            denom: invest.bond_denom,
        });
    }

    // calculate commission and remainer to unbond
    let commission = amount * invest.commission;

    // burn from the original caller
    execute_burn(deps.branch(), env.clone(), info.clone(), amount)?;

    if commission > Uint128::zero() {
        let sub_info = MessageInfo {
            sender: env.contract.address.clone(),
            funds: vec![],
        };

        // call cw20 spec mint tokens to owner, only the contract call
        execute_mint(
            deps.branch(),
            env.clone(),
            sub_info,
            invest.owner.to_string(),
            commission,
        )?;
    }

    // re-calculate bonded to ensure real values
    // bonded is the total number of tokens delegated from this address
    let bonded = get_bounded(&deps.querier, &env.contract.address)?;
    println!(
        "Current bonded of native tokens is: {:?} in unbond function",
        bonded
    );

    // calculate how many native tokens worth and update supply
    let remainer = amount.checked_sub(commission).map_err(StdError::overflow)?;

    let mut supply = TOTAL_SUPPLY.load(deps.storage)?;
    println!(
        "Current total supply of derivation tokens is: {:?} before unbond in unbond function",
        supply
    );

    assert_bonds(&supply, bonded)?;

    let unbond = remainer.multiply_ratio(bonded, supply.issued);
    supply.bonded = bonded.checked_sub(unbond).map_err(StdError::overflow)?;

    supply.issued = supply
        .issued
        .checked_sub(remainer)
        .map_err(StdError::overflow)?;

    supply.claims += unbond;

    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    println!(
        "Current total supply of derivation tokens is: {:?} after unbond in unbond function",
        supply
    );

    CLAIMS.create_claim(
        deps.storage,
        &info.sender,
        unbond,
        invest.unbonding_period.after(&env.block),
    )?;

    // unbond
    let resp = Response::new()
        .add_message(StakingMsg::Undelegate {
            validator: invest.validator,
            amount: coin(unbond.u128(), invest.bond_denom),
        })
        .add_attribute("action", "unbond")
        .add_attribute("to", info.sender)
        .add_attribute("unbonded", unbond)
        .add_attribute("burn", amount);

    Ok(resp)
}

pub fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // find how many tokens the contract has
    let invest = INVESTMENT.load(deps.storage)?;
    let mut balance = deps
        .querier
        .query_balance(&env.contract.address, &invest.bond_denom)?;

    if balance.amount < invest.min_withdrawal {
        return Err(ContractError::BalanceTooSmall {});
    }

    // check how much to send - min(balance, claims[sender]), and recude the claim
    // Ensure enough balance to cover this and only send some claims if that is all can cover
    let to_send =
        CLAIMS.claim_tokens(deps.storage, &info.sender, &env.block, Some(balance.amount))?;

    if to_send == Uint128::zero() {
        return Err(ContractError::NothingToClaim {});
    }

    // update total supply
    TOTAL_SUPPLY.update(deps.storage, |mut supply| -> StdResult<_> {
        supply.claims = supply.claims.checked_sub(to_send)?;
        Ok(supply)
    })?;

    // transfer tokens to the sender
    balance.amount = to_send;

    let resp = Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![balance],
        })
        .add_attribute("action", "claim")
        .add_attribute("from", info.sender)
        .add_attribute("amount", to_send);

    Ok(resp)
}

/// Reinvest will withdraw all pending rewards
/// then issue a callback to  itself  via bond_all_tokens to reinvest the new earnings (and anything else that accumulated)
pub fn reinvest(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address;
    let invest = INVESTMENT.load(deps.storage)?;
    let msg = to_binary(&ExecuteMsg::BondAllTokens {})?;

    // and bond them to the validator
    let resp = Response::new()
        .add_message(DistributionMsg::WithdrawDelegatorReward {
            validator: invest.validator,
        })
        .add_message(WasmMsg::Execute {
            contract_addr: contract_addr.to_string(),
            msg,
            funds: vec![],
        });

    Ok(resp)
}

pub fn bond_all_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }

    // find how many tokens to bond
    let invest = INVESTMENT.load(deps.storage)?;
    println!("Current investment is: {:?}", invest);

    let mut balance = deps
        .querier
        .query_balance(&env.contract.address, &invest.bond_denom)?;

    println!(
        "Current bonded of contract amount is: {} before bond all tokens",
        balance
    );

    // deduct pending claims from our account balance before reinvesting
    // if not enough funds, return a no-op
    match TOTAL_SUPPLY.update(deps.storage, |mut supply| -> StdResult<_> {
        println!("Current supply is: {:?}", supply);

        balance.amount = balance.amount.checked_sub(supply.claims)?;

        let min_withdrawal = invest.min_withdrawal;

        println!("current min widthrawal is: {}", min_withdrawal);
        println!(
            "balance of contract before sub min widthrawal is : {}",
            balance
        );

        balance.amount.checked_sub(min_withdrawal)?; // why balance doesn't decrement?

        supply.bonded += balance.amount;

        Ok(supply)
    }) {
        Ok(s) => {
            println!(
                "Current total supply of derivation tokens after updated is: {:?}",
                s
            );
        }
        Err(StdError::Overflow { .. }) => return Ok(Response::new()),
        Err(e) => return Err(ContractError::Std(e)),
    }

    println!("after calculate with supply, balance is: {:?}", balance);

    let resp = Response::new()
        .add_message(StakingMsg::Delegate {
            validator: invest.validator,
            amount: balance.clone(),
        })
        .add_attribute("action", "reinvest")
        .add_attribute("bonded", balance.amount);

    Ok(resp)
}

/// Returns the total amount of delegations from contract
/// it ensures they are all the same denom
pub fn get_bounded(querier: &QuerierWrapper, contract: &Addr) -> Result<Uint128, ContractError> {
    let bonds = querier.query_all_delegations(contract)?;

    if bonds.is_empty() {
        return Ok(Uint128::zero());
    }

    let denom = bonds[0].amount.denom.as_str();
    bonds.iter().fold(Ok(Uint128::zero()), |racc, d| {
        let acc = racc?;
        if d.amount.denom.as_str() != denom {
            Err(ContractError::DifferentBondDenom {
                denom1: denom.into(),
                denom2: d.amount.denom.to_string(),
            })
        } else {
            Ok(acc + d.amount.amount)
        }
    })
}

pub fn assert_bonds(supply: &Supply, bonded: Uint128) -> Result<(), ContractError> {
    if supply.bonded != bonded {
        Err(ContractError::BondedMismatch {
            stored: supply.bonded,
            queried: bonded,
        })
    } else {
        Ok(())
    }
}
