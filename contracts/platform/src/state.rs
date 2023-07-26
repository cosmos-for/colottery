use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use lottery::state::{LotteryPeriod, PlayerInfo, WinnerSelection};

#[cw_serde]
pub struct State {
    pub name: String,
    pub height: u64,
    pub created_at: Timestamp,
    pub created_by: Addr,
    pub lottery_code_id: u64,
    pub lotteries_count: u64,
    // pub players_count: u64,
}

impl State {
    pub fn new(
        name: String,
        height: u64,
        created_at: Timestamp,
        created_by: Addr,
        lottery_code_id: u64,
    ) -> Self {
        Self {
            name,
            height,
            created_at,
            created_by,
            lottery_code_id,
            lotteries_count: 0,
            // players_count: 0,
        }
    }
}

#[cw_serde]
pub struct LotteryInfo {
    pub name: String,
    pub symbol: String,
    pub height: u64,
    pub created_at: Timestamp,
    pub unit_price: Uint128,
    pub period: LotteryPeriod,
    pub selection: WinnerSelection,
    pub max_players: u32,
    pub contract_addr: Addr,
}

/// Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const STATE: Item<State> = Item::new("state");
pub const LOTTERIES: Map<&Addr, LotteryInfo> = Map::new("lotteries"); // (lottery address, lottery info)
                                                                      // pub const PLAYERS: Map<&Addr, PlayerInfo> = Map::new("players");    // (player address, playing info)

/// Cache lottery info
pub const PENDING_LOTTERY: Item<LotteryInfo> = Item::new("pending_lottery");
