use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Empty, Uint128};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw_utils::Expiration;

use crate::{state::Metadata, ContractError, Extension};

#[cw_serde]
pub enum ExecuteMsg {
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
    LoadFreight {
        token_id: String,
        denom: String,
        amount: Uint128,
        unit_weight: Uint128,
    },
    FuelUp {
        token_id: String,
        amount: Uint128,
    },
    BurnFuel {
        token_id: String,
        amount: Uint128,
    },
    UnloadFreight {
        token_id: String,
        denom: String,
        amount: Uint128,
    },
    DecreaseHealth {
        token_id: String,
        value: Uint128,
    },
}

impl TryFrom<ExecuteMsg> for Cw721ExecuteMsg<Metadata, Empty> {
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
            _ => Err(ContractError::UnimplementedErr {}),
        }
    }
}
