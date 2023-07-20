use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, DepsMut, DistributionMsg, Empty, Env, MessageInfo,
    QuerierWrapper, Response, StakingMsg, StdError, StdResult, Uint128, WasmMsg,
};

use cw721_base::Cw721Contract;

use crate::{msg::ExecuteMsg, ContractError, Extension};

type Cw721BaseContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

pub trait BaseExecute {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError>;
}

impl<'a> BaseExecute for Cw721BaseContract<'a> {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let cw721_msg = msg.try_into()?;

        let execute_res = self.execute(deps, env, info, cw721_msg);

        match execute_res {
            Ok(res) => Ok(res),
            Err(err) => Err(ContractError::try_from(err)?),
        }
    }
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    todo!()
}
