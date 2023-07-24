use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Empty};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw_utils::Expiration;

use crate::{state::Metadata, ContractError};

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
    Send {
        contract: String,
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
}

impl TryFrom<ExecuteMsg> for Cw721ExecuteMsg<Metadata, Empty> {
    type Error = ContractError;

    fn try_from(msg: ExecuteMsg) -> Result<Self, Self::Error> {
        use ExecuteMsg::*;

        match msg {
            // SendNft {
            //     contract,
            //     token_id,
            //     msg,
            // } => Ok(Cw721ExecuteMsg::SendNft {
            //     contract,
            //     token_id,
            //     msg,
            // }),
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
            ApproveAll { operator, expires } => {
                Ok(Cw721ExecuteMsg::ApproveAll { operator, expires })
            }
            RevokeAll { operator } => Ok(Cw721ExecuteMsg::RevokeAll { operator }),
            _ => Err(ContractError::UnimplementedErr {}),
        }
    }
}
