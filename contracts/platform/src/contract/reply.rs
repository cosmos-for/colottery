use cosmwasm_std::{attr, DepsMut, Env, Reply, Response, StdError, SubMsgResponse};
use cw_utils::parse_instantiate_response_data;

use crate::{
    state::{LOTTERIES, PENDING_LOTTERY, STATE},
    ContractError,
};

use super::{CREATE_LOTTERY_REPLY_ID, DRAW_LOTTERY_REPLY_ID};

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        CREATE_LOTTERY_REPLY_ID => {
            initial_lottery_instantiated(deps, env, reply.result.into_result())
        }
        DRAW_LOTTERY_REPLY_ID => lottery_drawed(deps, env, reply.result.into_result()),
        id => Err(ContractError::UnRecognizedReplyId { id }),
    }
}

pub fn initial_lottery_instantiated(
    deps: DepsMut,
    _env: Env,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    // Parse data from reply
    let resp = reply.map_err(StdError::generic_err)?;
    let data = resp.data.ok_or(ContractError::DataMissing {})?;
    let resp = parse_instantiate_response_data(&data)?;

    let lottery_addr = &deps.api.addr_validate(&resp.contract_address)?;

    let mut lottery = PENDING_LOTTERY.load(deps.storage)?;
    lottery.contract_addr = lottery_addr.to_owned();

    LOTTERIES.save(deps.storage, lottery_addr, &lottery)?;

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.lotteries_count += 1;
        Ok(state)
    })?;

    let attrs = vec![attr("action", "reply_create_lottery")];

    Ok(Response::new().add_attributes(attrs))
}

pub fn lottery_drawed(
    deps: DepsMut,
    env: Env,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
