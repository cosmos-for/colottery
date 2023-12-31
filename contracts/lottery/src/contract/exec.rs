use cosmwasm_std::{
    attr, coins, to_binary, Addr, BankMsg, DepsMut, Env, MessageInfo, Response, Storage, WasmMsg,
};

use cw_storage_plus::Map;

use crate::{
    auth::exec::{
        validate_balance, validate_buy, validate_double_buy, validate_draw, validate_owner,
    },
    hash,
    msg::{ExecuteMsg, QueryMsg},
    state::{
        GameStatus, PlayerInfo, State, WinnerInfo, IDX_2_ADDR, OWNER, PLAYERS, PLAYER_COUNTER,
        STATE,
    },
    ContractError, Cw721MetadataContract, Extension,
};

pub trait BaseExecute {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError>;
}

impl<'a> BaseExecute for Cw721MetadataContract<'a> {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let cw721_msg = msg.try_into()?;

        // println!("executing msg {:?} in base execute", cw721_msg);
        let execute_resp = self.execute(deps, env, info, cw721_msg);
        // println!("executed msg response {:?} in base execute", execute_resp);

        match execute_resp {
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

    let contract = Cw721MetadataContract::default();

    match msg {
        BuyTicket { denom, memo } => buy_ticket(deps, &env, &info, &denom, memo),
        DrawLottery {} => draw_lottery(deps, &env, &info),
        ClaimLottery {} => claim_lottery(deps, &env, &info),
        WithdrawFunds {
            amount,
            denom,
            recipient,
        } => withdraw(deps, &env, &info, amount, denom.as_str(), recipient),
        Transfer { recipient } => transfer(deps, &env, &info, recipient),
        _ => contract.base_execute(deps, env, info, msg),
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

    // mint nft
    let token_id = &state.player_count.to_string();
    let resp = mint_nft(env, token_id, sender, None, Default::default())?;

    let attributes = vec![
        attr("action", "buy_ticket"),
        attr("sender", sender.as_str()),
        attr("denom", denom),
    ];

    Ok(resp.add_attributes(attributes))
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

    state.seed = hash::finalize(&state.seed, sender, env.block.height, &transaction);

    let winners = choose_winner_infos(deps.storage, PLAYERS, IDX_2_ADDR, &state, player_counter)?;

    if winners.is_empty() {
        state.winner = vec![];
    } else {
        let balances = deps
            .querier
            .query_balance(&env.contract.address, &state.unit_price.denom)?;
        let winner_player = winners.first().unwrap();
        let winner_info = WinnerInfo {
            address: winner_player.player_addr.clone(),
            prize: vec![balances],
            ticket_id: winner_player.ticket_id.clone(),
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

pub fn claim_lottery(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let state = STATE.load(deps.storage)?;

    // check the ticket' owner is sender
    let ticket_id = state.winner.first().unwrap().ticket_id.clone();

    let ticket: cw721::OwnerOfResponse = deps.querier.query_wasm_smart(
        env.contract.address.as_str(),
        &QueryMsg::OwnerOf {
            token_id: ticket_id,
            include_expired: Some(true),
        },
    )?;

    // check lottery is closed and sender is winner
    if state.is_closed() && ticket.owner == *sender {
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
        let seed_num = hash::hash_to_u64(seed);

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

    state.seed = hash::update(&state.seed, sender, player_counter, current_height, &memo);

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
            ticket_id: player_counter.to_string(),
            memo,
        },
    )?;

    PLAYER_COUNTER.save(deps.storage, &player_counter)?;

    IDX_2_ADDR.save(deps.storage, player_counter, sender)?;

    Ok(())
}

pub fn mint_nft(
    env: &Env,
    token_id: &str,
    sender: &Addr,
    token_uri: Option<String>,
    extension: Extension,
) -> Result<Response, ContractError> {
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.to_owned(),
        owner: sender.to_string(),
        token_uri,
        extension,
    };

    let msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    };

    let attrs = vec![
        attr("action", "mint_nft"),
        attr("reciever", sender.as_str()),
        attr("token_id", token_id),
    ];

    Ok(Response::new().add_attributes(attrs).add_message(msg))
}
