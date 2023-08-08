use cosmwasm_std::{ensure, Addr, Coin, Deps, Env, MessageInfo, Storage, Uint128};
use cw_storage_plus::Map;
use cw_utils::must_pay;

use crate::{
    state::{LotteryCategory, PlayerInfo, State, PLAYER_COUNTER},
    ContractError,
};

pub type UnitResult = Result<(), ContractError>;

pub fn validate_buy(
    storage: &dyn Storage,
    state: &State,
    info: &MessageInfo,
    denom: &str,
    env: &Env,
) -> UnitResult {
    let amount = must_pay(info, denom)?;

    validate_winner_selection(state)?;

    validate_denom(state, denom)?;

    validate_price(state, amount)?;

    validate_player_counter(storage, state)?;

    validate_status(state)?;

    validate_timestamp(state, env)
}

pub fn validate_draw(
    state: &State,
    owner: &Addr,
    info: &MessageInfo,
    env: &Env,
    player_counter: u64,
) -> UnitResult {
    validate_owner(owner, info)?;

    validate_height(state, env)?;

    validate_timestamp_or_activing(state, env, player_counter)?;

    validate_status(state)
}

pub fn validate_set_prizes(state: &State, owner: &Addr, info: &MessageInfo) -> UnitResult {
    validate_owner(owner, info)?;
    validate_prepare_category(state)
}

pub fn validate_winner_selection(state: &State) -> UnitResult {
    ensure!(
        state.selection.is_jackpot(),
        ContractError::UnSupportedWinnerSelection {
            selection: state.selection.clone()
        }
    );
    Ok(())
}

pub fn validate_denom(state: &State, denom: &str) -> UnitResult {
    ensure!(
        denom == state.unit_price.denom,
        ContractError::UnSupportedDenom {
            denom: denom.into(),
        }
    );
    Ok(())
}

pub fn validate_price(state: &State, payment_amount: Uint128) -> UnitResult {
    ensure!(
        payment_amount >= state.unit_price.amount,
        ContractError::PaymentNotEnough {
            amount: payment_amount
        }
    );
    Ok(())
}

pub fn validate_player_counter(storage: &dyn Storage, state: &State) -> UnitResult {
    let player_counter = PLAYER_COUNTER.load(storage)?;
    ensure!(
        player_counter < state.max_players,
        ContractError::PlayerExceededMaximum {
            max_players: player_counter,
        }
    );

    Ok(())
}

pub fn validate_timestamp(state: &State, env: &Env) -> UnitResult {
    let current_time = env.block.time;
    ensure!(
        current_time <= state.expiratoin,
        ContractError::AlreadyExpired {}
    );

    Ok(())
}

pub fn validate_height(state: &State, env: &Env) -> UnitResult {
    let current_height = env.block.height;
    let lottery_height = state.height;

    // Only can buy lottery after created block height
    ensure!(
        current_height >= state.height,
        ContractError::LotteryHeightNotMatch {
            current_height,
            lottery_height,
        }
    );

    Ok(())
}

pub fn validate_status(state: &State) -> UnitResult {
    // Can't buy lottery after lottery is already closed
    ensure!(!state.is_closed(), ContractError::LotteryAlreadyClosed {});

    Ok(())
}

pub fn validate_owner(owner: &Addr, info: &MessageInfo) -> UnitResult {
    ensure!(owner == info.sender, ContractError::Unauthorized {});

    Ok(())
}

pub fn validate_timestamp_or_activing(state: &State, env: &Env, player_counter: u64) -> UnitResult {
    ensure!(
        env.block.time >= state.expiratoin || player_counter >= state.player_count,
        ContractError::LotteryIsActiving {}
    );

    Ok(())
}

pub fn validate_balance(balance: &Coin, to_withdraw: u128) -> UnitResult {
    ensure!(
        balance.amount.u128() >= to_withdraw,
        ContractError::BalanceTooSmall {
            balance: balance.to_owned()
        }
    );
    Ok(())
}

pub fn validate_double_buy(
    deps: Deps,
    players: Map<&Addr, PlayerInfo>,
    sender: &Addr,
) -> UnitResult {
    let player_info = players.may_load(deps.storage, sender)?;
    ensure!(
        player_info.is_none(),
        ContractError::LotteryCanBuyOnce {
            player: sender.clone(),
        }
    );

    Ok(())
}

pub fn validate_prepare_category(state: &State) -> UnitResult {
    let category = &state.category;

    ensure!(
        *category == LotteryCategory::SpecifyPrize {},
        ContractError::InvalidCategory {
            value: category.to_string(),
        }
    );

    Ok(())
}
