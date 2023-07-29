use common::hash::{self, hash_to_u64};
use cosmwasm_std::{attr, coins, Addr, BankMsg, DepsMut, Env, MessageInfo, Response, Storage};

use cw_storage_plus::Map;

use crate::{
    auth::exec::{
        validate_balance, validate_buy, validate_double_buy, validate_draw, validate_owner,
    },
    msg::ExecuteMsg,
    state::{
        GameStatus, PlayerInfo, State, WinnerInfo, IDX_2_ADDR, OWNER, PLAYERS, PLAYER_COUNTER,
        STATE,
    },
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
    let mut state = STATE.load(deps.storage)?;

    validate_buy(deps.storage, &state, info, denom, env)?;

    let sender = &info.sender;

    validate_double_buy(deps.as_ref(), PLAYERS, sender)?;

    update_state_with_buy(deps, env, &mut state, sender, memo)?;

    let attributes = vec![
        attr("action", "buy_ticket"),
        attr("sender", sender.as_str()),
        attr("denom", denom),
    ];

    Ok(Response::new().add_attributes(attributes))
}

pub fn draw_lottery(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

    let mut state = STATE.load(deps.storage)?;

    let owner = OWNER.load(deps.storage)?;
    let player_counter = PLAYER_COUNTER.load(deps.storage)?;

    validate_draw(&state, &owner, info, env, player_counter)?;

    let current_height = env.block.height;
    let transaction = env.transaction.as_ref().map(|t| t.index.to_string());

    // Change status to `Closed`
    state.status = GameStatus::Closed;

    state.seed = hash::seed::finalize(&state.seed, sender, env.block.height, &transaction);

    let winner = choose_winner_infos(deps.storage, PLAYERS, IDX_2_ADDR, &state, player_counter)?;

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

    // check lottery is closed and sender is winner
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

    validate_owner(&owner, info)?;

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

    validate_owner(&owner, info)?;

    let balance = deps.querier.query_balance(&env.contract.address, denom)?;

    validate_balance(&balance, amount)?;

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

// Choose winner use random number in players
pub fn choose_winner_infos(
    storage: &dyn Storage,
    players: Map<&Addr, PlayerInfo>,
    idx_addr: Map<u64, Addr>,
    state: &State,
    player_counter: u64,
) -> Result<Vec<PlayerInfo>, ContractError> {
    // validate_winner_selection(state)?;

    if state.player_count == 0 {
        Ok(vec![])
    } else if state.player_count == 1 {
        let winner = players.first(storage)?;
        Ok(winner.into_iter().map(|(_, player)| player).collect())
    } else {
        let seed = state.seed.as_str();
        let seed_num = hash_to_u64(seed);

        println!("The seed num is {:?}", seed_num);

        let idx = seed_num % player_counter + 1;
        println!("The idx is: {:?}", idx);

        let address = idx_addr.may_load(storage, idx)?.unwrap();
        let player_info = players.load(storage, &address)?;
        Ok(vec![player_info])
    }
}

fn update_state_with_buy(
    deps: DepsMut,
    env: &Env,
    state: &mut State,
    sender: &Addr,
    memo: Option<String>,
) -> Result<(), ContractError> {
    let current_height = env.block.height;
    let lottery_addr = &env.contract.address;

    let player_counter = PLAYER_COUNTER.load(deps.storage)? + 1;

    state.seed = hash::seed::update(&state.seed, sender, player_counter, current_height, &memo);

    state.player_count += 1;

    STATE.save(deps.storage, state)?;

    PLAYERS.save(
        deps.storage,
        sender,
        &PlayerInfo {
            player_addr: sender.clone(),
            lottery_addr: lottery_addr.to_owned(),
            height: current_height,
            buy_at: current_height,
            memo,
        },
    )?;

    PLAYER_COUNTER.save(deps.storage, &player_counter)?;

    IDX_2_ADDR.save(deps.storage, player_counter, sender)?;

    Ok(())
}
