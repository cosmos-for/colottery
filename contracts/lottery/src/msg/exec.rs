use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin};
use cw_utils::Expiration;

use crate::{ContractError, Cw721ExecuteMsg, Extension};

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    BuyTicket {
        denom: String,
        memo: Option<String>,
    },
    DrawLottery {
        // lottery: String,
    },
    ClaimLottery {},
    WithdrawFunds {
        amount: u128,
        denom: String,
        recipient: Option<String>,
    },
    Transfer {
        recipient: String,
    },
    /// 设置预定奖项
    SetPrizes {
        prizes: Vec<PreparePrize>,
    },

    // NFT msg
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    Mint {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Extension,
    },
    Burn {
        token_id: String,
    },
    SetMinter {
        minter: String,
    },
}

#[cw_serde]
pub struct PreparePrize {
    pub prize: Coin,
    pub count: u64,
    pub level: u64,
}

impl TryFrom<ExecuteMsg> for Cw721ExecuteMsg {
    type Error = ContractError;

    fn try_from(msg: ExecuteMsg) -> Result<Self, Self::Error> {
        use ExecuteMsg::*;

        match msg {
            TransferNft {
                recipient,
                token_id,
            } => Ok(Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            }),
            Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => Ok(Cw721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            }),
            SendNft {
                contract,
                token_id,
                msg,
            } => Ok(Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            }),
            Approve {
                spender,
                token_id,
                expires,
            } => Ok(Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            }),
            Revoke { spender, token_id } => Ok(Cw721ExecuteMsg::Revoke { spender, token_id }),
            Burn { token_id } => Ok(Cw721ExecuteMsg::Burn { token_id }),
            ApproveAll { operator, expires } => {
                Ok(Cw721ExecuteMsg::ApproveAll { operator, expires })
            }
            RevokeAll { operator } => Ok(Cw721ExecuteMsg::RevokeAll { operator }),
            _ => Err(ContractError::Unimplemented {}),
        }
    }
}
