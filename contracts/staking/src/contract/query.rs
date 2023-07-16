use cosmwasm_std::{coin, to_binary, Binary, Decimal, Deps, Env, StdResult};

use cw20_base::{
    allowances::query_allowance,
    contract::{query_balance, query_token_info},
};
use cw_controllers::ClaimsResponse;

use crate::{
    msg::{InvestmentResponse, QueryMsg},
    state::{CLAIMS, INVESTMENT, TOTAL_SUPPLY},
};

const FALLBACK_RATIO: Decimal = Decimal::one();

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Claims { address } => query_claims(deps, address).and_then(|r| to_binary(&r)),
        Investment {} => query_investment(deps).and_then(|r| to_binary(&r)),
        Balance { address } => query_balance(deps, address).and_then(|r| to_binary(&r)),
        TokenInfo {} => query_token_info(deps).and_then(|r| to_binary(&r)),
        Allowance { owner, spender } => {
            query_allowance(deps, owner, spender).and_then(|r| to_binary(&r))
        }
    }
}

pub fn query_claims(deps: Deps, address: String) -> StdResult<ClaimsResponse> {
    CLAIMS.query_claims(deps, &deps.api.addr_validate(&address)?)
}

pub fn query_investment(deps: Deps) -> StdResult<InvestmentResponse> {
    let investment = INVESTMENT.load(deps.storage)?;
    let supply = TOTAL_SUPPLY.load(deps.storage)?;
    let nominal_value = if supply.issued.is_zero() {
        FALLBACK_RATIO
    } else {
        Decimal::from_ratio(supply.bonded, supply.issued)
    };

    let resp = InvestmentResponse {
        owner: investment.owner.into(),
        commission: investment.commission,
        validator: investment.validator,
        min_withdrawal: investment.min_withdrawal,
        token_supply: supply.issued,
        staked_tokens: coin(supply.bonded.u128(), investment.bond_denom),
        nominal_value,
    };

    Ok(resp)
}
