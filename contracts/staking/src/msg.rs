use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use cw_utils::Duration;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    /// Decimal place of the derivative token
    pub decimals: u8,

    /// Validator that all tokens will be bonded to
    pub validator: String,
    // the unbonding period of the native staking module
    pub unbonding_period: Duration,

    /// commission for staking when someone unbonds
    pub commission: Decimal,
    pub min_withdrawal: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
