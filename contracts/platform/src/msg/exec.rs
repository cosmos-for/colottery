use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use lottery::state::WinnerSelection;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    CreateLottery {
        name: String,
        symbol: String,
        unit_price_amount: u128,
        unit_price_denom: String,
        period: String,
        expiration: u64,
        selection: WinnerSelection,
        max_players: u64,
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
