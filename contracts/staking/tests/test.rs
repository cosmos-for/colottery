use cosmwasm_std::{
    coin, coins,
    testing::{mock_dependencies, mock_env, mock_info, MockQuerier, MOCK_CONTRACT_ADDR},
    Addr, Coin, CosmosMsg, Decimal, Deps, Env, FullDelegation, StakingMsg, Uint128, Validator,
};

use cw20_base::contract::{query_balance, query_token_info};
use cw_controllers::Claim;
use cw_utils::{Duration, DAY, HOUR, WEEK};
use staking::{
    contract::{exec::execute, instantiate, query::query_investment},
    msg::{ExecuteMsg, InstantiateMsg},
    state::CLAIMS,
    ContractError,
};

use std::str::FromStr;

const DEFAULT_VALIDATOR: &str = "default-validator";
const NATIVE_DENOM: &str = "ustake"; // also the staking denom
const DERIVATION_DENOM: &str = "cdr";

fn sample_validator(addr: &str) -> Validator {
    Validator {
        address: addr.into(),
        commission: Decimal::percent(3),
        max_commission: Decimal::percent(10),
        max_change_rate: Decimal::percent(1),
    }
}

fn sample_delegation(addr: &str, amount: Coin) -> FullDelegation {
    let can_redelegate = amount.clone();
    let accumulated_rewards = coins(0, &amount.denom);

    FullDelegation {
        delegator: Addr::unchecked(MOCK_CONTRACT_ADDR),
        validator: addr.into(),
        amount,
        can_redelegate,
        accumulated_rewards,
    }
}

fn set_validator(querier: &mut MockQuerier) {
    querier.update_staking(NATIVE_DENOM, &[sample_validator(DEFAULT_VALIDATOR)], &[]);
}

fn set_delegation(querier: &mut MockQuerier, amount: u128, denom: &str) {
    querier.update_staking(
        NATIVE_DENOM,
        &[sample_validator(DEFAULT_VALIDATOR)],
        &[sample_delegation(DEFAULT_VALIDATOR, coin(amount, denom))],
    )
}

fn later(env: &Env, delta: Duration) -> Env {
    let time_delta = match delta {
        Duration::Time(t) => t,
        _ => panic!("Must provide durationin time"),
    };

    let mut resp = env.clone();
    resp.block.time = resp.block.time.plus_seconds(time_delta);
    resp
}

fn default_instantiate(commission: u64, min_withdrawal: u128) -> InstantiateMsg {
    InstantiateMsg {
        name: "CoolDollar".to_string(),
        symbol: DERIVATION_DENOM.to_string(),
        decimals: 9,
        validator: DEFAULT_VALIDATOR.into(),
        unbonding_period: DAY * 3,
        commission: Decimal::percent(commission),
        min_withdrawal: Uint128::from(min_withdrawal),
    }
}

fn get_balance<U: Into<String>>(deps: Deps, addr: U) -> Uint128 {
    query_balance(deps, addr.into()).unwrap().balance
}

fn get_claims(deps: Deps, addr: &str) -> Vec<Claim> {
    CLAIMS
        .query_claims(deps, &Addr::unchecked(addr))
        .unwrap()
        .claims
}

#[test]
fn instantiation_with_missing_validator_should_fails() {
    let mut deps = mock_dependencies();
    deps.querier
        .update_staking(NATIVE_DENOM, &[sample_validator("john")], &[]);

    let creator = String::from("creator");

    let msg = InstantiateMsg {
        name: "CoolD".to_string(),
        symbol: DERIVATION_DENOM.to_string(),
        decimals: 9,
        validator: String::from("validator1"), // "validator1" does not exist
        unbonding_period: WEEK,
        commission: Decimal::percent(2),
        min_withdrawal: Uint128::new(50),
    };

    let info = mock_info(&creator, &[]);

    let err = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NoInValidatorSet {
            validator: "validator1".to_string()
        }
    )
}

#[test]
fn instantiate_should_works() {
    let mut deps = mock_dependencies();
    deps.querier.update_staking(
        NATIVE_DENOM,
        &[
            sample_validator("alice"),
            sample_validator("bob"),
            sample_validator("Charlie"),
        ],
        &[],
    );

    let creator = String::from("creator");
    let msg = InstantiateMsg::new("CoolDollar", "CD", 0, "bob", HOUR * 12, 2, 50);

    let info = mock_info(&creator, &[]);

    let resp = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
    assert_eq!(0, resp.messages.len());

    // token info
    let token = query_token_info(deps.as_ref()).unwrap();
    assert_eq!(&token.name, &msg.name);
    assert_eq!(&token.symbol, &msg.symbol);
    assert_eq!(&token.decimals, &msg.decimals);
    assert_eq!(token.total_supply, Uint128::zero());

    // no balance
    assert_eq!(get_balance(deps.as_ref(), &creator), Uint128::zero());
    // no claims
    assert_eq!(get_claims(deps.as_ref(), &creator), vec![]);

    // investment info
    let investment = query_investment(deps.as_ref()).unwrap();
    assert_eq!(investment.owner, Addr::unchecked(&creator));
    assert_eq!(investment.commission, msg.commission);
    assert_eq!(investment.validator, msg.validator);
    assert_eq!(investment.min_withdrawal, msg.min_withdrawal);

    assert_eq!(investment.token_supply, Uint128::zero());
    assert_eq!(investment.staked_tokens, coin(0, NATIVE_DENOM));
    assert_eq!(investment.nominal_value, Decimal::one());
}

#[test]
fn bonding_issues_tokens_should_work() {
    let mut deps = mock_dependencies();
    set_validator(&mut deps.querier);

    let creator = String::from("creator");
    let instantiate_msg = default_instantiate(2, 50);
    let info = mock_info(&creator, &[]);

    // instantiate
    let resp = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, resp.messages.len());

    // bond
    let bob = "bob";
    let bond_msg = ExecuteMsg::Bond {};
    let info = mock_info(bob, &[coin(10, "random"), coin(1000, NATIVE_DENOM)]);

    let resp = execute(deps.as_mut(), mock_env(), info, bond_msg).unwrap();

    assert_eq!(1, resp.messages.len());

    let deletage = &resp.messages[0];

    match &deletage.msg {
        CosmosMsg::Staking(StakingMsg::Delegate { validator, amount }) => {
            assert_eq!(validator.as_str(), DEFAULT_VALIDATOR);
            assert_eq!(amount, &coin(1000, NATIVE_DENOM));
        }
        _ => panic!("Unexpected message: {:?}", deletage),
    }

    // bob got 1000 DRV for 1000 stake at 1.0 ratio
    assert_eq!(get_balance(deps.as_ref(), bob), Uint128::new(1000));

    // investment info
    let invest = query_investment(deps.as_ref()).unwrap();

    assert_eq!(invest.token_supply, Uint128::new(1000));
    assert_eq!(invest.staked_tokens, coin(1000, NATIVE_DENOM));
    assert_eq!(invest.nominal_value, Decimal::one());

    // token info
    let token = query_token_info(deps.as_ref()).unwrap();
    assert_eq!(token.total_supply, Uint128::new(1000));
    assert_eq!(token.symbol, DERIVATION_DENOM);
    assert_eq!(token.decimals, 9);
}

#[test]
fn rebonding_changes_pricing_should_works() {
    let mut deps = mock_dependencies();
    set_validator(&mut deps.querier);

    let creator = "creator";
    let init_msg = default_instantiate(2, 50);
    let info = mock_info(creator, &[]);

    instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

    // bond
    let bob = "bob";
    let bond_msg = ExecuteMsg::Bond {};
    let info = mock_info(bob, &[coin(10, "random"), coin(1000, NATIVE_DENOM)]);
    let resp = execute(deps.as_mut(), mock_env(), info, bond_msg).unwrap();

    assert_eq!(1, resp.messages.len());

    // update the querier with now bond
    set_delegation(&mut deps.querier, 1000, NATIVE_DENOM);

    // fake a reinvestment
    let rebond_msg = ExecuteMsg::BondAllTokens {};
    let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
    deps.querier
        .update_balance(MOCK_CONTRACT_ADDR, coins(2000, NATIVE_DENOM));

    // execute bond all tokens
    let _ = execute(deps.as_mut(), mock_env(), info, rebond_msg).unwrap();

    // update the querier with new bond
    set_delegation(&mut deps.querier, 3000, NATIVE_DENOM);

    // now see 1000 issued(derivation token) and 3000 bonded(native token)

    let invest = query_investment(deps.as_ref()).unwrap();
    assert_eq!(invest.token_supply, Uint128::new(1000)); // token_supply (derivation token) is only about `Bond` operator, supply has nothing to do 'with set_delegation'
    assert_eq!(invest.staked_tokens, coin(3000, NATIVE_DENOM));

    let ratio = Decimal::from_str("3").unwrap();
    assert_eq!(invest.nominal_value, ratio);

    // bond some other tokens from alice and get a different issuance price (maintaining the ratio)
    let alice = "alice";
    let bond_msg = ExecuteMsg::Bond {};
    let info = mock_info(alice, &[coin(3000, NATIVE_DENOM)]);
    let resp = execute(deps.as_mut(), mock_env(), info, bond_msg).unwrap();
    assert_eq!(1, resp.messages.len());

    // update the querier with new bond
    set_delegation(&mut deps.querier, 6000, NATIVE_DENOM);

    // alice shoul gotten 1000 DRV for the
    assert_eq!(get_balance(deps.as_ref(), alice), Uint128::new(1000));

    let invest = query_investment(deps.as_ref()).unwrap();
    assert_eq!(invest.token_supply, Uint128::new(2000));
    assert_eq!(invest.staked_tokens, coin(6000, NATIVE_DENOM));
    assert_eq!(invest.nominal_value, ratio);
}
