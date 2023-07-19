use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw20::{AllowanceResponse, BalanceResponse, TokenInfoResponse};
use cw_controllers::ClaimsResponse;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
