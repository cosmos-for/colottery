use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::{
    msg::InstantiateMsg,
    state::{State, OWNER, STATE},
    ContractError,
};

use super::{CONTRACT_NAME, CONTRACT_VERSION};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;
    let state = State::new(
        msg.name,
        env.block.height,
        env.block.time,
        sender.to_owned(),
    );

    STATE.save(deps.storage, &state)?;
    OWNER.save(deps.storage, sender)?;

    let attributes = vec![
        attr("action", "instantitate_platform"),
        attr("sender", sender),
    ];

    Ok(Response::new().add_attributes(attributes))
}
