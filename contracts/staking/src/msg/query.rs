use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw20::{AllowanceResponse, BalanceResponse, TokenInfoResponse};
use cw_controllers::ClaimsResponse;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// `Claims` shows the number of the tokens by the given address can withdraw when they are unbonding
    #[returns(ClaimsResponse)]
    Claims { address: String },
    /// `Investment` shows metadata on the staking info of the contract
    #[returns(InvestmentResponse)]
    Investment {},

    /// CW20 spec, Return the current balance of the given address, 0 if unset
    #[returns(BalanceResponse)]
    Balance { address: String },

    #[returns(TokenInfoResponse)]
    TokenInfo {},

    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
}

#[cw_serde]
pub struct InvestmentResponse {
    pub token_supply: Uint128,
    pub staked_tokens: Coin,
    // Ratio of staked_token / token_supply
    pub nominal_value: Decimal,

    pub owner: String,
    pub commission: Decimal,
    pub validator: String,
    pub min_withdrawal: Uint128,
}
