pub mod auth;
pub mod contract;
mod error;

pub mod msg;
pub mod state;

#[cfg(any(feature = "mt", test))]
pub mod multitest;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use state::Metadata;

pub use crate::error::ContractError;

pub type Extension = Metadata;

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub const ARCH_DEMON: &str = "aconst";
pub const ARCH_DECIMALS: u8 = 18;

pub fn support_coins() -> Vec<SupportCoin> {
    vec![SupportCoin::new(ARCH_DEMON, ARCH_DECIMALS)]
}

#[cw_serde]
pub struct SupportCoin {
    pub denom: String,
    pub decimals: u8,
}

impl SupportCoin {
    pub fn new(denom: impl Into<String>, decimals: u8) -> Self {
        Self {
            denom: denom.into(),
            decimals,
        }
    }
}
