use cosmwasm_std::{DepsMut, Env, Reply, Response};

use crate::ContractError;

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    // match reply.id {
    //     INITIAL_LOTTERY_INSTANTIATION_REPLY_ID => reply::initial_lottery_instantiated(
    //         deps,
    //         env,
    //         reply.result.into_result(),
    //         LOTTERIES,
    //         CONFIG,
    //         LATEST_LOTTERY,
    //     ),
    //     CLOSE_LOTTERY_REPLY_ID => reply::closed_lottery(deps, env, reply.result.into_result()),
    //     id => Err(ContractError::UnRecognizedReplyIdErr { id }),
    // }

    Ok(Response::new())
}
