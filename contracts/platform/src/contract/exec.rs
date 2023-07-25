use cosmwasm_std::{attr, coins, Addr, BankMsg, DepsMut, Env, MessageInfo, Response, Storage};

use cw_storage_plus::Map;
use cw_utils::must_pay;

use crate::{
    msg::ExecuteMsg,
    state::{OWNER, PLAYERS, STATE},
    ContractError, ARCH_DEMON,
};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    // TODO
    match msg {
        BuyTicket {
            lottery,
            denom,
            memo,
        } => buy_ticket(deps, env, info, &denom, memo),
        DrawLottery { lottery } => draw_lottery(deps, env, info),
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
    Ok(Response::new())
}

pub fn draw_lottery(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let sender = info.sender;

    let mut state = STATE.load(deps.storage)?;

    let owner = OWNER.load(deps.storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    let current_height = env.block.height;
    let lottery_height = state.height;

    STATE.save(deps.storage, &state)?;

    let attributes = vec![
        attr("action", "draw_lottery"),
        attr("sender", sender.as_str()),
        attr("height", current_height.to_string()),
    ];

    Ok(Response::new().add_attributes(attributes))
}
