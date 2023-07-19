use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, DepsMut, DistributionMsg, Env, MessageInfo, QuerierWrapper,
    Response, StakingMsg, StdError, StdResult, Uint128, WasmMsg,
};

use crate::{msg::ExecuteMsg, ContractError};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    todo!()
}
