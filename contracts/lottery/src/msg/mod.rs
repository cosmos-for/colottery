pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symobl: String,
    pub unit_price: Uint128,
    pub period: String,
}
