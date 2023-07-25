use cosmwasm_std::{
    attr, to_binary, Addr, DepsMut, Env, MessageInfo, Response, SubMsg, Timestamp, Uint128, WasmMsg,
};

use lottery::msg::InstantiateMsg as LotteryInstantiateMsg;
use lottery::state::WinnerSelection;

use crate::state::{LotteryInfo, LOTTERIES, PENDING_LOTTERY};
use crate::{
    msg::ExecuteMsg,
    state::{OWNER, PLAYERS, STATE},
    ContractError,
};

use super::CREATE_LOTTERY_REPLY_ID;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    // TODO
    match msg {
        CreateLottery {
            name,
            symobl,
            unit_price,
            period,
            selection,
            max_players,
            label,
        } => create_lottery(
            deps,
            &env,
            &info,
            &name,
            &symobl,
            unit_price,
            &period,
            selection,
            max_players,
            &label,
        ),
        BuyLottery {
            lottery,
            denom,
            memo,
        } => buy_lottery(deps, &env, &info, &lottery, &denom, memo),
        DrawLottery { lottery } => draw_lottery(deps, &env, &info, &lottery),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_lottery(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    name: &str,
    symobl: &str,
    unit_price: Uint128,
    period: &str,
    selection: WinnerSelection,
    max_players: u32,
    label: &str,
) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let state = STATE.load(deps.storage)?;

    let init_lottery_msg = LotteryInstantiateMsg::new(
        name,
        symobl,
        unit_price,
        period,
        selection.clone(),
        max_players,
    );

    let msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: state.lottery_code_id,
        msg: to_binary(&init_lottery_msg)?,
        funds: vec![],
        label: label.to_owned(),
    };

    let msg = SubMsg::reply_on_success(msg, CREATE_LOTTERY_REPLY_ID);
    let attrs = vec![attr("action", "create_lottery"), attr("sender", sender)];

    let lottery = LotteryInfo {
        name: name.to_owned(),
        symbol: symobl.to_owned(),
        height: env.block.height,
        created_at: env.block.time,
        unit_price,
        period: period.parse()?,
        selection,
        contract_addr: Addr::unchecked(""), // update by reply
    };

    PENDING_LOTTERY.save(deps.storage, &lottery)?;

    Ok(Response::new().add_submessage(msg).add_attributes(attrs))
}

pub fn buy_lottery(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    lottery: &str,
    denom: &str,
    memo: Option<String>,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn draw_lottery(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    lottery: &str,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

    let owner = OWNER.load(deps.storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new())
}
