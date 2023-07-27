pub mod contract;
mod error;

pub mod msg;
pub mod state;

#[cfg(any(feature = "mt", test))]
pub mod multitest;

pub use crate::error::ContractError;

pub const ARCH_DEMON: &str = "aconst";
pub const ARCH_DECIMALS: u8 = 18;
