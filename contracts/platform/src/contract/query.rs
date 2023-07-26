use cosmwasm_std::{to_binary, Binary, Deps, Env, Order, StdResult};

use crate::{
    msg::{CurrentStateResp, OwnerResp, PlayersResp, QueryMsg},
    state::{OWNER, PLAYERS, STATE},
};

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => owner(deps),
        QueryMsg::CurrentState {} => current_state(deps),
        QueryMsg::Balances {} => balances(deps, &env),
        QueryMsg::Players {} => players(deps),
    }
}

pub fn owner(deps: Deps) -> StdResult<Binary> {
    let owner = OWNER.load(deps.storage)?;
    to_binary(&OwnerResp { owner })
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

pub fn players(deps: Deps) -> StdResult<Binary> {
    let players: StdResult<Vec<_>> = PLAYERS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let players = players?.into_iter().map(|(_, player)| player).collect();
    to_binary(&PlayersResp { players })
}
