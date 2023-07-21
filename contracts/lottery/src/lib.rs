pub mod contract;
mod error;

pub mod msg;
pub mod state;

#[cfg(any(feature = "mt", test))]
pub mod multitest;

use cosmwasm_std::Empty;
use state::Metadata;

pub use crate::error::ContractError;

pub type Extension = Metadata;

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub const NATIVE_DENOM: &str = "ARCH";
