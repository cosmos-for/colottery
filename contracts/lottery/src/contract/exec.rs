use cosmwasm_std::{Addr, DepsMut, Empty, Env, MessageInfo, Response, Storage};

use cw721_base::Cw721Contract;
use cw_storage_plus::Map;
use cw_utils::must_pay;

use crate::{
    msg::ExecuteMsg,
    state::{GameStatus, PlayerInfo, WinnerInfo, OWNER, PLAYERS, STATE},
    ContractError, Extension, ARCH_DEMON,
};

type Cw721BaseContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

pub trait BaseExecute {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError>;
}

impl<'a> BaseExecute for Cw721BaseContract<'a> {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let cw721_msg = msg.try_into()?;

        let execute_res = self.execute(deps, env, info, cw721_msg);

        match execute_res {
            Ok(res) => Ok(res),
            Err(err) => Err(ContractError::try_from(err)?),
        }
    }
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        BuyTicket { denom, memo } => buy_ticket(deps, env, info, &denom, memo),
        DrawLottery {} => draw_lottery(deps, env, info),
        // WithdrawRewards { amount, denom } => {
        //     withdraw(deps, env, info, amount, denom.as_str(), STATE, WITHDRAWS)
        // }
        // Transfer { recipient } => transfer(deps, env, info, recipient, STATE),
        _ => unimplemented!(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn buy_ticket(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: &str,
    memo: Option<String>,
) -> Result<Response, ContractError> {
    // Check funds pay, only support ARCH first
    if denom != ARCH_DEMON {
        return Err(ContractError::UnSupportedDenom {
            denom: denom.into(),
        });
    }

    let amount = must_pay(&info, denom)?;

    let mut state = STATE.load(deps.storage)?;

    if amount < state.unit_price {
        return Err(ContractError::PaymentNotEnough { amount });
    }

    let lottery_sequnce = state.height;

    let contract_addr = env.contract.address;
    let block_height = env.block.height;

    // Only can buy lottery after created block height
    if state.height > block_height {
        return Err(ContractError::LotteryHeightNotMatch {
            current_height: block_height,
            lottery_height: lottery_sequnce,
        });
    }

    // Can't buy lottery after lottery is already closed
    if state.is_closed() {
        return Err(ContractError::LotteryAlreadyClosed {
            address: contract_addr,
        });
    }

    let sender = info.sender;
    let player = PLAYERS.may_load(deps.storage, &sender)?;

    // Only can buy lottery once
    match player {
        Some(_) => Err(ContractError::LotteryCanBuyOnce {
            player: sender,
            lottery: contract_addr,
        }),
        None => {
            state.player_count += 1;
            STATE.save(deps.storage, &state)?;

            PLAYERS.save(
                deps.storage,
                &sender,
                &PlayerInfo {
                    address: sender.clone(),
                    height: block_height,
                    buy_at: block_height,
                    memo,
                },
            )?;
            Ok(Response::new())
        }
    }
}

pub fn draw_lottery(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let sender = info.sender;

    let mut state = STATE.load(deps.storage)?;

    let owner = OWNER.load(deps.storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    if state.is_closed() {
        return Err(ContractError::LotteryAlreadyClosed {
            address: env.contract.address,
        });
    }

    let current_height = env.block.height;
    let lottery_height = state.height;

    // Only can buy lottery after created block height
    if lottery_height > current_height {
        return Err(ContractError::LotteryHeightNotMatch {
            current_height,
            lottery_height,
        });
    }

    let winner = choose_winner_infos(PLAYERS, deps.storage)?;

    if winner.is_empty() {
        state.winner = vec![];
    } else {
        let balances = deps
            .querier
            .query_balance(env.contract.address, ARCH_DEMON)?;
        let winner_info = WinnerInfo {
            address: winner.first().as_ref().unwrap().address.clone(),
            prize: vec![balances],
        };
        state.winner.push(winner_info);
    }

    // Change status to `Closed`
    state.status = GameStatus::Closed;

    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

// pub fn transfer(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     recipient: String,
//     state_item: Item<State>,
// ) -> Result<Response, ContractError> {
//     let sender = info.sender;
//     let mut state = state_item.load(deps.storage)?;

//     if sender != state.owner {
//         return Err(ContractError::UnauthorizedErr {});
//     }

//     let owner = deps.api.addr_validate(&recipient)?;

//     state.owner = owner;

//     state_item.save(deps.storage, &state)?;

//     Ok(Response::new())
// }

// pub fn withdraw(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     amount: u128,
//     denom: &str,
//     state_item: Item<State>,
//     withdraws: Map<&Addr, Vec<Coin>>,
// ) -> Result<Response, ContractError> {
//     let sender = info.sender;
//     let state = state_item.load(deps.storage)?;

//     if sender != state.owner {
//         return Err(ContractError::UnauthorizedErr {});
//     }

//     let balance = deps.querier.query_balance(env.contract.address, denom)?;
//     if balance.amount.u128() < amount {
//         return Err(ContractError::WidthrawAmountTooMuchErr {
//             amount,
//             denom: denom.into(),
//         });
//     }

//     let ws = withdraws.may_load(deps.storage, &sender)?;

//     let mut ws = ws.unwrap_or_default();
//     let withdraw_coin = coin(amount, denom);
//     ws.push(withdraw_coin.clone());

//     withdraws.save(deps.storage, &sender, &ws)?;

//     let bank_msg = BankMsg::Send {
//         to_address: sender.as_str().into(),
//         amount: vec![withdraw_coin],
//     };

//     Ok(Response::new()
//         .add_message(bank_msg)
//         .add_attribute("action", "withdraw")
//         .add_attribute("sender", sender.as_str()))
// }

// TODO Choose winner use random number in players
pub fn choose_winner_infos(
    players: Map<&Addr, PlayerInfo>,
    storage: &dyn Storage,
) -> Result<Vec<PlayerInfo>, ContractError> {
    // TODO get first
    let winner = players.first(storage)?;
    Ok(winner.into_iter().map(|(_, player)| player).collect())
}
