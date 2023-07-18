use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{validator} is not in validator set")]
    NoInValidatorSet { validator: String },

    #[error("{denom}'s balance is empty")]
    EmptyBalance { denom: String },

    #[error("Bond denom: {denom1} and {denom2} is different")]
    DifferentBondDenom { denom1: String, denom2: String },

    #[error("The bond: {stored} and {queried} is mismatch")]
    BondedMismatch { stored: Uint128, queried: Uint128 },

    #[error("Unbond amount: {min_bonded}{denom} is too small")]
    UnbondTooSmall { min_bonded: Uint128, denom: String },

    #[error("The contract balance is too small")]
    BalanceTooSmall {},

    #[error("The contract has nothing to claim")]
    NothingToClaim {},

    /// for cw20 spec
    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid expiration")]
    InvalidExpiration {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},
}

impl From<cw20_base::ContractError> for ContractError {
    fn from(err: cw20_base::ContractError) -> Self {
        use cw20_base::ContractError::*;

        match err {
            Std(error) => ContractError::Std(error),
            Unauthorized {} => ContractError::Unauthorized {},
            CannotSetOwnAccount {} => ContractError::CannotSetOwnAccount {},
            InvalidExpiration {} => ContractError::InvalidExpiration {},
            // InvalidZeroAmount {  }
            Expired {} => ContractError::Expired {},
            NoAllowance {} => ContractError::NoAllowance {},
            CannotExceedCap {} => ContractError::CannotExceedCap {},
            DuplicateInitialBalanceAddresses {} => {
                ContractError::DuplicateInitialBalanceAddresses {}
            }
            _ => ContractError::Std(StdError::generic_err(err.to_string())),
        }
    }
}
