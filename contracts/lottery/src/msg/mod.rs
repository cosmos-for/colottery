pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;

use crate::state::WinnerSelection;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symobl: String,
    pub unit_price_amount: u128,
    pub unit_price_denom: String,
    pub period: String,
    pub selection: WinnerSelection,
    pub max_players: u32,
}

impl InstantiateMsg {
    pub fn new(
        name: impl Into<String>,
        symobl: impl Into<String>,
        unit_price_amount: u128,
        unit_price_denom: impl Into<String>,
        period: impl Into<String>,
        selection: WinnerSelection,
        max_players: u32,
    ) -> Self {
        Self {
            name: name.into(),
            symobl: symobl.into(),
            unit_price_amount,
            unit_price_denom: unit_price_denom.into(),
            period: period.into(),
            selection,
            max_players,
        }
    }
}
