use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response};

use cw721_base::Cw721Contract;
use cw_utils::must_pay;

use crate::{
    msg::ExecuteMsg,
    state::{PlayerInfo, PLAYERS, STATE},
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
        // Draw {} => exec::draw(deps, env, info, STATE, BETTORS),
        // WithdrawRewards { amount, denom } => {
        //     exec::withdraw(deps, env, info, amount, denom.as_str(), STATE, WITHDRAWS)
        // }
        // Transfer { recipient } => exec::transfer(deps, env, info, recipient, STATE),
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
        return Err(ContractError::NotSupportDenom {
            denom: denom.into(),
        });
    }

    let amount = must_pay(&info, denom)?;

    let state = STATE.load(deps.storage)?;

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

// pub fn draw(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     state_item: Item<State>,
//     bettors: Map<&Addr, BetInfo>,
// ) -> Result<Response, ContractError> {
//     let sender = info.sender;
//     let rewards = info.funds;
//     let mut state = state_item.load(deps.storage)?;

//     if state.owner != sender {
//         return Err(ContractError::UnauthorizedErr {});
//     }

//     let block_height = env.block.height;
//     let lottery_sequnce = state.height;

//     // Only can buy lottery after created block height
//     if state.height > block_height {
//         return Err(ContractError::LotterySequenceNotMatchErr {
//             height: block_height,
//             sequence: lottery_sequnce,
//         });
//     }

//     // Can't buy lottery after lottery is already closed
//     if state.winner.is_some() {
//         return Err(ContractError::LotteryIsAlreadyClosedErr {
//             addr: env.contract.address,
//         });
//     }

//     let winner = choose_winner(bettors, deps.storage)?;

//     ensure!(winner.is_some(), ContractError::LotteryNoBettorErr {});

//     // Set the rewards
//     state.rewards = rewards;

//     state.owner = winner.clone().unwrap();

//     // Choose the winner
//     state.winner = winner;

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

// pub fn choose_winner(
//     bettors: Map<&Addr, BetInfo>,
//     storage: &dyn Storage,
// ) -> Result<Option<Addr>, ContractError> {
//     let winner = bettors.first(storage)?;
//     Ok(winner.map(|(k, _)| k))
// }
