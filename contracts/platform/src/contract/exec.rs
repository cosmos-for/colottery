use cosmwasm_std::coin;
use cosmwasm_std::{attr, to_binary, Addr, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};

use lottery::msg::ExecuteMsg as LotteryExecuteMsg;
use lottery::msg::InstantiateMsg as LotteryInstantiateMsg;
use lottery::state::WinnerSelection;

use crate::state::{LotteryInfo, PENDING_LOTTERY};
use crate::{
    msg::ExecuteMsg,
    state::{OWNER, STATE},
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

    match msg {
        CreateLottery {
            name,
            symbol: symobl,
            unit_price_amount,
            unit_price_denom,
            period,
            expiration,
            selection,
            max_players,
            category,
            label,
        } => create_lottery(
            deps,
            &env,
            &info,
            &name,
            &symobl,
            unit_price_amount,
            &unit_price_denom,
            &period,
            expiration,
            selection,
            max_players,
            category,
            &label,
        ),

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
    unit_price_amount: u128,
    unit_price_denom: &str,
    period: &str,
    expiration: u64,
    selection: WinnerSelection,
    max_players: u64,
    category: Option<String>,
    label: &str,
) -> Result<Response, ContractError> {
    let sender = &info.sender;
    let state = STATE.load(deps.storage)?;

    let init_lottery_msg = LotteryInstantiateMsg::new(
        name,
        symobl,
        unit_price_amount,
        unit_price_denom,
        period,
        expiration,
        selection.clone(),
        max_players,
        category,
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
        unit_price: coin(unit_price_amount, unit_price_denom),
        period: period.parse()?,
        selection,
        max_players,
        contract_addr: Addr::unchecked(""), // update by reply
    };

    PENDING_LOTTERY.save(deps.storage, &lottery)?;

    Ok(Response::new().add_submessage(msg).add_attributes(attrs))
}

// pub fn buy_lottery(
//     _deps: DepsMut,
//     _env: &Env,
//     info: MessageInfo,
//     lottery: &str,
//     denom: &str,
//     memo: Option<String>,
// ) -> Result<Response, ContractError> {
//     let buy_msg = LotteryExecuteMsg::BuyTicket { denom: denom.into(), memo };
//     let msg = WasmMsg::Execute {
//         contract_addr: lottery.to_string(),
//         msg: to_binary(&buy_msg)?,
//         funds: info.funds,
//     };

//     let attrs = vec![attr("action", "buy_lottery"), attr("sender", info.sender)];

//     Ok(Response::new().add_message(msg).add_attributes(attrs))
// }

pub fn draw_lottery(
    deps: DepsMut,
    _env: &Env,
    info: &MessageInfo,
    lottery: &str,
) -> Result<Response, ContractError> {
    let sender = &info.sender;

    let owner = OWNER.load(deps.storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg = LotteryExecuteMsg::DrawLottery {};
    let msg = WasmMsg::Execute {
        contract_addr: lottery.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    let attrs = vec![
        attr("action", "draw_lottery"),
        attr("sender", info.sender.as_str()),
    ];

    Ok(Response::new().add_message(msg).add_attributes(attrs))
}
