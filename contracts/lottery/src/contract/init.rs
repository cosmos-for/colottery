use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};

use crate::{
    msg::InstantiateMsg,
    state::{InvestmentInfo, Supply, INVESTMENT, TOTAL_SUPPLY},
    ContractError,
};

use super::{CONTRACT_NAME, CONTRACT_VERSION};
// use cw2::set_contract_version;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}
