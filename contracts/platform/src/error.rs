use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

use lottery::ContractError as LotteryContractError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{id} is an unknown reply id")]
    UnRecognizedReplyId { id: u64 },

    #[error("Data missing")]
    DataMissing {},

    #[error("{0}")]
    ParseErr(#[from] ParseReplyError),

    #[error("{0}")]
    LotteryContractErr(#[from] LotteryContractError),
}
