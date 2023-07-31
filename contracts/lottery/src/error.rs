use cosmwasm_std::{Addr, Coin, StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

use crate::{msg::QueryMsg, state::WinnerSelection};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{value} is not a valid lottery period format")]
    InvalidLottoryPeriod { value: String },

    #[error("{value} is not a valid unit price")]
    InvalidUnitPrice { value: u128 },

    #[error("Operation not implemented")]
    Unimplemented {},

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
    BalanceTooSmall { balance: Coin },

    #[error("Not support denom: {denom}")]
    UnSupportedDenom { denom: String },

    #[error("Unsupport selection: {:?}", selection)]
    UnSupportedWinnerSelection { selection: WinnerSelection },

    #[error("{player} Only can buy a lottery once")]
    LotteryCanBuyOnce { player: Addr },

    #[error("The payment funds is not enough")]
    PaymentNotEnough { amount: Uint128 },

    #[error("The lottery player is full: {max_players}")]
    PlayerExceededMaximum { max_players: u64 },

    #[error("The lottery is activing, can't draw")]
    LotteryIsActiving {},

    #[error("The expiration: {expiration_secs} in instantiate message is not valid, must greater than {created_at}")]
    InstantiateExpirationInvalid {
        expiration_secs: u64,
        created_at: u64,
    },

    #[error("error(0)")]
    PaymentError(#[from] PaymentError),

    #[error(
        "Current height: {current_height} must greater than lottery start height: {lottery_height}"
    )]
    LotteryHeightNotMatch {
        current_height: u64,
        lottery_height: u64,
    },

    #[error("Lottery is already closed")]
    LotteryAlreadyClosed {},

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

    // for 721
    #[error("token_id already claimed")]
    AlreadyClaimed {},

    #[error("Expired")]
    AlreadyExpired {},

    #[error("NotFound")]
    NotFound {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("Unsupported query message: {:?}", msg)]
    QueryMsgNotSupported { msg: QueryMsg },
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

impl TryFrom<cw721_base::ContractError> for ContractError {
    type Error = ContractError;

    fn try_from(err: cw721_base::ContractError) -> Result<Self, Self::Error> {
        use cw721_base::ContractError::*;

        match err {
            // Unauthorized {} => Ok(ContractError::UnauthorizedErr {}),
            Claimed {} => Ok(ContractError::AlreadyClaimed {}),
            Expired {} => Ok(ContractError::AlreadyExpired {}),
            ApprovalNotFound { spender } => Ok(ContractError::ApprovalNotFound { spender }),
            _ => Err(ContractError::Unimplemented {}),
        }
    }
}
