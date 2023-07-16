use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};

use crate::{
    msg::InstantiateMsg,
    state::{InvestmentInfo, Supply, INVESTMENT, TOTAL_SUPPLY},
    ContractError,
};

use super::{CONTRACT_NAME, CONTRACT_VERSION};
// use cw2::set_contract_version;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // ensure the validator is registered
    let vals = deps.querier.query_all_validators()?;
    if !vals.iter().any(|v| v.address == msg.validator) {
        return Err(ContractError::NoInValidatorSet {
            validator: msg.validator,
        });
    }

    // store token info using  CW20-base format
    let token = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: Uint128::zero(),
        mint: Some(MinterData {
            minter: env.contract.address,
            cap: None,
        }),
    };

    TOKEN_INFO.save(deps.storage, &token)?;

    let denom = deps.querier.query_bonded_denom()?;

    let invest = InvestmentInfo {
        owner: info.sender,
        commission: msg.commission,
        unbonding_period: msg.unbonding_period,
        min_withdrawal: msg.min_withdrawal,
        bond_denom: denom,
        validator: msg.validator,
    };

    INVESTMENT.save(deps.storage, &invest)?;

    let supply = Supply::default();

    TOTAL_SUPPLY.save(deps.storage, &supply)?;

    Ok(Response::new())
}
