use cosmwasm_std::{to_binary, Binary, Coin, Deps, Env, StdResult};

use cw721_base::entry::query as cw721_query;

use crate::{
    msg::{CurrentStateResp, OwnerResp, PlayInfoResp, QueryMsg, WinnerResp},
    state::{OWNER, PLAYERS, STATE},
};

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => owner(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::Winner {} => winner(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::CurrentState {} => current_state(deps).and_then(|resp| to_binary(&resp)),
        QueryMsg::Balances {} => balances(deps, &env).and_then(|cs| to_binary(&cs)),
        QueryMsg::PlayInfo { address } => {
            play_info(deps, &address).and_then(|info| to_binary(&info))
        }

        _ => {
            let query_msg = msg.into();
            cw721_query(deps, env, query_msg)
        }
    }
}

pub fn owner(deps: Deps) -> StdResult<OwnerResp> {
    let owner = OWNER.load(deps.storage)?;
    Ok(OwnerResp { owner })
}

pub fn winner(deps: Deps) -> StdResult<WinnerResp> {
    let state = STATE.load(deps.storage)?;
    Ok(WinnerResp {
        winner: state.winner,
    })
}

pub fn current_state(deps: Deps) -> StdResult<CurrentStateResp> {
    let state = STATE.load(deps.storage)?;
    Ok(CurrentStateResp { state })
}

pub fn balances(deps: Deps, env: &Env) -> StdResult<Vec<Coin>> {
    deps.querier.query_all_balances(&env.contract.address)
}

pub fn play_info(deps: Deps, address: &str) -> StdResult<PlayInfoResp> {
    let address = deps.api.addr_validate(address)?;
    let player = PLAYERS.may_load(deps.storage, &address)?;
    Ok(PlayInfoResp { info: player })
}
