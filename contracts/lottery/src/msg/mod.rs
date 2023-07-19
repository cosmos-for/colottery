pub mod exec;
pub mod query;

pub use exec::*;
pub use query::*;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};
use cw_utils::Duration;

#[cw_serde]
pub struct InstantiateMsg {}
