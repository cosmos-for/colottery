use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BettorCountResp)]
    BettorCount {},
    #[returns(PrizeAmountResp)]
    PrizeAmount {},
    #[returns(WinnerResp)]
    Winner {},
}

#[cw_serde]
pub struct BettorCountResp {
    pub counter: u64,
}

#[cw_serde]
pub struct PrizeAmountResp {
    pub amount: Coin,
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}

#[cw_serde]
pub struct WinnerResp {
    pub winner: Option<Addr>,
}
