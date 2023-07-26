use cosmwasm_schema::cw_serde;

#[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum ExecuteMsg {
    BuyTicket {
        lottery: String,
        denom: String,
        memo: Option<String>,
    },
    DrawLottery {
        lottery: String,
    },
}
