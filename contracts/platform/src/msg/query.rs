use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use lottery::state::PlayerInfo;

use crate::state::State;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PlayersResp)]
    Players {},
    #[returns(BalancesResp)]
    Balances {},
    #[returns(OwnerResp)]
    Owner {},
    #[returns(CurrentStateResp)]
    CurrentState {},
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
pub struct CurrentStateResp {
    pub state: State,
}

#[cw_serde]
pub struct OwnerResp {
    pub owner: Addr,
}

#[cw_serde]
pub struct PlayersResp {
    pub players: Vec<PlayerInfo>,
}
