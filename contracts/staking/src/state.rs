use std::collections::HashSet;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_controllers::Claims;
use cw_storage_plus::Item;
use cw_utils::Duration;

/// Investment config information for the contract
#[cw_serde]
pub struct InvestmentInfo {
    /// Owner created the contract and take a cut
    pub owner: Addr,
    ///  Denomination that can stake (only 1 first, and can add other denominations later)
    pub bond_denom: String,
    /// This is the unbonding period of the native token staking module
    /// We need this to only allow claims to be redeemed after the money has arrived.
    pub unbonding_period: Duration,
    /// commission for staking when someone unbond
    pub commission: Decimal,
    /// All tokens are bonded to a Validator
    pub validator: String,
    /// Minimum withdraw amount
    pub min_withdrawal: Uint128,
}

/// Supply is dynamic and tracks the current supply of staked and cw20 tokens
#[cw_serde]
#[derive(Default)]
pub struct Supply {
    /// `issued` is how many derivative tokens have been issued by this contract
    pub issued: Uint128,
    /// `bonded` is how many native tokens are currently bonded to a validator
    pub bonded: Uint128,
    /// `reserved` is how many token need to be reserved paying back
    pub reserved: Uint128,
}

/// Storage
pub const CLAIMS: Claims = Claims::new("claims");

pub const INVESTMENT: Item<InvestmentInfo> = Item::new("investment");
pub const TOTAL_SUPPLY: Item<Supply> = Item::new("total_supply");
