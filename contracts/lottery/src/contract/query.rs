use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

use crate::{
    msg::{CurrentStateResp, OwnerResp, PlayInfoResp, QueryMsg, WinnerResp},
    state::{OWNER, PLAYERS, STATE},
};

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => owner(deps),
        QueryMsg::Winner {} => winner(deps),
        QueryMsg::CurrentState {} => current_state(deps),
        QueryMsg::Balances {} => balances(deps, &env),
        QueryMsg::PlayInfo { address } => play_info(deps, &address),
    }
}

pub fn owner(deps: Deps) -> StdResult<Binary> {
    let owner = OWNER.load(deps.storage)?;
    to_binary(&OwnerResp { owner })
}

pub fn winner(deps: Deps) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;
    to_binary(&WinnerResp {
        winner: state.winner,
    })
}

pub fn current_state(deps: Deps) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;
    to_binary(&CurrentStateResp { state })
}

pub fn balances(deps: Deps, env: &Env) -> StdResult<Binary> {
    deps.querier
        .query_all_balances(&env.contract.address)
        .and_then(|balances| to_binary(&balances))
}

pub fn play_info(deps: Deps, address: &str) -> StdResult<Binary> {
    let address = deps.api.addr_validate(address)?;
    let player = PLAYERS.may_load(deps.storage, &address)?;
    to_binary(&PlayInfoResp { info: player })
}
