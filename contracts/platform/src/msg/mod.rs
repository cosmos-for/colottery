pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
}

impl InstantiateMsg {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
