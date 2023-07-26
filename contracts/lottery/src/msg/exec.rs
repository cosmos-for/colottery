use cosmwasm_schema::cw_serde;

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
}

// impl TryFrom<ExecuteMsg> for Cw721ExecuteMsg<Metadata, Empty> {
//     type Error = ContractError;

//     fn try_from(msg: ExecuteMsg) -> Result<Self, Self::Error> {
//         use ExecuteMsg::*;

//         match msg {
//             // SendNft {
//             //     contract,
//             //     token_id,
//             //     msg,
//             // } => Ok(Cw721ExecuteMsg::SendNft {
//             //     contract,
//             //     token_id,
//             //     msg,
//             // }),
//             Approve {
//                 spender,
//                 token_id,
//                 expires,
//             } => Ok(Cw721ExecuteMsg::Approve {
//                 spender,
//                 token_id,
//                 expires,
//             }),
//             Revoke { spender, token_id } => Ok(Cw721ExecuteMsg::Revoke { spender, token_id }),
//             ApproveAll { operator, expires } => {
//                 Ok(Cw721ExecuteMsg::ApproveAll { operator, expires })
//             }
//             RevokeAll { operator } => Ok(Cw721ExecuteMsg::RevokeAll { operator }),
//             _ => Err(ContractError::UnimplementedErr {}),
//         }
//     }
// }
