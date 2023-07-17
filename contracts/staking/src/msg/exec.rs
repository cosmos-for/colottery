use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint128};
use cw_utils::Expiration;

#[cw_serde]
pub enum ExecuteMsg {
    /// `Bond` will bond all staking tokens sent with the message and release derivation tokens
    Bond {},
    /// `Unbond` will "burn" the given amount of derivation tokens
    /// and send the unbounded staking tokens back to the sender (after decuted commission)
    Unbond {
        amount: Uint128,
    },
    /// `Claim` your native tokens that already unbounded, Only can do this after the unbonding period (e.g. 3 weeks)
    Claim {},
    /// `Reinvest` will check for all accumulated staking tokens rewards, withdraw them
    /// and re-bond them to the same validator. Anyone can do this, which update the value of tokens under the custody.
    Reinvest {},
    /// `_BondAllTokens` can only be called by the contract itself, after all rewards have been withdraw.
    /// This can only be invoked by the contract itself as a return from reinvest
    BondAllTokens {},

    /// CW20 spec
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    Burn {
        amount: Uint128,
    },
    /// `Send` a message to transfer tokens to a contract and trigger an action on the receiving contract.
    /// The msg must be a valid Binary message
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// extend cw20 spec, Allow spender to access an additional amount tokens from the owner's (info.sender) account.
    /// If expires is set, overrides current allowance expiration with it.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },

    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    BurnFrom {
        owner: String,
        amount: Uint128,
    },
}
