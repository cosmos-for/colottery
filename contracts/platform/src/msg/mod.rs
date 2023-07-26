pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub lottery_code_id: u64,
}

impl InstantiateMsg {
    pub fn new(name: impl Into<String>, lottery_code_id: u64) -> Self {
        Self {
            name: name.into(),
            lottery_code_id,
        }
    }
}
