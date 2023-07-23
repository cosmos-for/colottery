pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::state::WinnerSelection;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symobl: String,
    pub unit_price: Uint128,
    pub period: String,
    pub selection: WinnerSelection,
    pub max_bettors: u32,
}

impl InstantiateMsg {
    pub fn new(
        name: impl Into<String>,
        symobl: impl Into<String>,
        unit_price: Uint128,
        period: String,
        selection: WinnerSelection,
        max_bettors: u32,
    ) -> Self {
        Self {
            name: name.into(),
            symobl: symobl.into(),
            unit_price,
            period,
            selection,
            max_bettors,
        }
    }
}
