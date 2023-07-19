use cosmwasm_std::{coin, to_binary, Binary, Decimal, Deps, Env, StdResult};

use cw_controllers::ClaimsResponse;

use crate::msg::QueryMsg;

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    todo!()
}
