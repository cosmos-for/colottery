use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::{State, WinnerInfo};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalancesResp)]
    Balances {},
    #[returns(WinnerResp)]
    Winner {},
    #[returns(OwnerResp)]
    Owner {},
    #[returns(CurrentStateResp)]
    CurrentState {},
    #[returns(IsJoinedResp)]
    IsJoined { address: String },
}

#[cw_serde]
pub struct BalancesResp {
    pub amount: Coin,
}

// #[cw_serde]
// pub struct InstantiationData {
//     pub addr: Addr,
// }

#[cw_serde]
pub struct WinnerResp {
    pub winner: Vec<WinnerInfo>,
}

#[cw_serde]
pub struct OwnerResp {
    pub owner: Addr,
}

#[cw_serde]
pub struct CurrentStateResp {
    pub state: State,
}

#[cw_serde]
pub struct IsJoinedResp {
    pub joined: bool,
}
