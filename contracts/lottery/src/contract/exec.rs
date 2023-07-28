use cosmwasm_std::{
    attr, coins, Addr, BankMsg, DepsMut, Env, MessageInfo, Response, StdResult, Storage,
};

use cw_storage_plus::Map;
use cw_utils::must_pay;

use crate::{
    msg::ExecuteMsg,
    state::{GameStatus, PlayerInfo, WinnerInfo, OWNER, PLAYERS, PLAYER_COUNTER, STATE},
    ContractError, ARCH_DEMON,
};

// type Cw721BaseContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

// pub trait BaseExecute {
//     fn base_execute(
//         &self,
//         deps: DepsMut,
//         env: Env,
//         info: MessageInfo,
//         msg: ExecuteMsg,
//     ) -> Result<Response, ContractError>;
// }

// impl<'a> BaseExecute for Cw721BaseContract<'a> {
//     fn base_execute(
//         &self,
//         deps: DepsMut,
//         env: Env,
//         info: MessageInfo,
//         msg: ExecuteMsg,
//     ) -> Result<Response, ContractError> {
//         let cw721_msg = msg.try_into()?;

//         let execute_res = self.execute(deps, env, info, cw721_msg);

//         match execute_res {
//             Ok(res) => Ok(res),
//             Err(err) => Err(ContractError::try_from(err)?),
//         }
//     }
// }

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        BuyTicket { denom, memo } => buy_ticket(deps, &env, &info, &denom, memo),
        DrawLottery {} => draw_lottery(deps, &env, &info),
        ClaimLottery {} => claim_lottery(deps, &info),
        WithdrawFunds {
            amount,
            denom,
            recipient,
        } => withdraw(deps, &env, &info, amount, denom.as_str(), recipient),
        Transfer { recipient } => transfer(deps, &env, &info, recipient),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn buy_ticket(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    denom: &str,
    memo: Option<String>,
) -> Result<Response, ContractError> {
    // Check funds pay, only support ARCH first
    if denom != ARCH_DEMON {
        return Err(ContractError::UnSupportedDenom {
            denom: denom.into(),
        });
    }

    let amount = must_pay(info, denom)?;

    let mut state = STATE.load(deps.storage)?;

    if amount < state.unit_price.amount {
        return Err(ContractError::PaymentNotEnough { amount });
    }

    let player_counter = PLAYER_COUNTER.load(deps.storage)?;

    if player_counter == state.max_players {
        return Err(ContractError::PlayerExceededMaximum {
            max_players: player_counter,
        });
    }

    let lottery_height = state.height;

    let contract_addr = &env.contract.address;
    let current_height = env.block.height;

    // Only can buy lottery after created block height
    if state.height > current_height {
        return Err(ContractError::LotteryHeightNotMatch {
            current_height,
            lottery_height,
        });
    }

    if env.block.time >= state.expiratoin {
        return Err(ContractError::AlreadyExpired {});
    }

    // Can't buy lottery after lottery is already closed
    if state.is_closed() {
        return Err(ContractError::LotteryAlreadyClosed {
            address: contract_addr.to_owned(),
        });
    }

    let sender = &info.sender;
    let player = PLAYERS.may_load(deps.storage, sender)?;

    // Only can buy lottery once
    match player {
        Some(_) => Err(ContractError::LotteryCanBuyOnce {
            player: sender.clone(),
            lottery: contract_addr.to_owned(),
        }),
        None => {
            state.player_count += 1;
            STATE.save(deps.storage, &state)?;

            PLAYERS.save(
                deps.storage,
                sender,
                &PlayerInfo {
                    player_addr: sender.clone(),
                    lottery_addr: env.contract.address.clone(),
                    height: current_height,
                    buy_at: current_height,
                    memo,
                },
            )?;

            PLAYER_COUNTER.update(deps.storage, |c| -> StdResult<u32> { Ok(c + 1) })?;

            let attributes = vec![
                attr("action", "buy_ticket"),
                attr("sender", sender.as_str()),
                attr("denom", denom),
                attr("amount", amount.to_string()),
                attr("height", current_height.to_string()),
            ];

            Ok(Response::new().add_attributes(attributes))
        }
    }
}

pub fn draw_lottery(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

    let mut state = STATE.load(deps.storage)?;

    let owner = OWNER.load(deps.storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    if state.is_closed() {
        return Err(ContractError::LotteryAlreadyClosed {
            address: env.contract.address.to_owned(),
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

    let current_time = env.block.time;
    let player_counter = PLAYER_COUNTER.load(deps.storage)?;

    // if lottery is expired or player exceed maximum, lottery can be drawed
    if (current_time < state.expiratoin) && (player_counter < state.max_players) {
        return Err(ContractError::LotteryIsActiving {});
    }

    let winner = choose_winner_infos(PLAYERS, deps.storage)?;

    if winner.is_empty() {
        state.winner = vec![];
    } else {
        let balances = deps
            .querier
            .query_balance(&env.contract.address, ARCH_DEMON)?;
        let winner_info = WinnerInfo {
            address: winner.first().as_ref().unwrap().player_addr.clone(),
            prize: vec![balances],
        };
        state.winner.push(winner_info);
    }

    // Change status to `Closed`
    state.status = GameStatus::Closed;

    STATE.save(deps.storage, &state)?;

    let attributes = vec![
        attr("action", "draw_lottery"),
        attr("sender", sender.as_str()),
        attr("height", current_height.to_string()),
    ];

    Ok(Response::new().add_attributes(attributes))
}

pub fn claim_lottery(deps: DepsMut, info: &MessageInfo) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let state = STATE.load(deps.storage)?;

    if state.is_closed() && state.winner.first().map(|w| &w.address) == Some(sender) {
        OWNER.save(deps.storage, sender)?;

        let attributes = vec![
            attr("action", "claim_lottery"),
            attr("sender", sender.as_str()),
            attr("owner", sender),
        ];

        Ok(Response::new().add_attributes(attributes))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

pub fn transfer(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    recipient: String,
) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let owner = OWNER.load(deps.storage)?;

    if sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let recipient: Addr = deps.api.addr_validate(&recipient)?;
    let height = env.block.height;

    OWNER.save(deps.storage, &recipient)?;

    let attributes = vec![
        attr("action", "transfer_lottery"),
        attr("sender", sender.as_str()),
        attr("recipient", recipient.as_str()),
        attr("height", height.to_string()),
    ];

    Ok(Response::new().add_attributes(attributes))
}

pub fn withdraw(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    amount: u128,
    denom: &str,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

    let owner = OWNER.load(deps.storage)?;

    if sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let balance = deps.querier.query_balance(&env.contract.address, denom)?;
    if balance.amount.u128() < amount {
        return Err(ContractError::BalanceTooSmall { balance });
    }

    let recipient = recipient.unwrap_or(sender.to_string());

    let bank_msg = BankMsg::Send {
        to_address: recipient.clone(),
        amount: coins(amount, denom),
    };

    let attributes = vec![
        attr("action", "withdraw"),
        attr("sender", sender.as_str()),
        attr("recipient", recipient),
        attr("amount", amount.to_string()),
        attr("denom", denom),
    ];

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attributes(attributes))
}

// TODO Choose winner use random number in players
pub fn choose_winner_infos(
    players: Map<&Addr, PlayerInfo>,
    storage: &dyn Storage,
) -> Result<Vec<PlayerInfo>, ContractError> {
    // TODO get first
    let winner = players.first(storage)?;
    Ok(winner.into_iter().map(|(_, player)| player).collect())
}
