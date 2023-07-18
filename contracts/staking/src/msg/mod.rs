pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};
use cw_utils::Duration;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    /// symbol / ticker of the derivative token
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

impl InstantiateMsg {
    pub fn new(
        name: &str,
        symbol: &str,
        decimals: u8,
        validator: &str,
        unbonding_period: Duration,
        commssion: u64,
        min_withdrawal: u128,
    ) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            decimals,
            validator: validator.to_string(),
            unbonding_period,
            commission: Decimal::percent(commssion),
            min_withdrawal: Uint128::from(min_withdrawal),
        }
    }
}
