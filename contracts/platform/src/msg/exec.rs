use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use lottery::state::WinnerSelection;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    CreateLottery {
        name: String,
        symobl: String,
        unit_price: Uint128,
        period: String,
        selection: WinnerSelection,
        max_players: u32,
        label: String,
    },
    DrawLottery {
        lottery: String,
    },
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}
