#[cfg(test)]
mod tests;

use anyhow::Result as AnyResult;

use cosmwasm_std::{Addr, Coin, StdResult, Uint128};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query, reply},
    msg::*,
};

#[derive(Clone, Debug, Copy)]
pub struct LotteryCodeId(u64);

impl LotteryCodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        let code_id = app.store_code(Box::new(contract));
        Self(code_id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: Addr,
        name: &str,
        label: &str,
    ) -> AnyResult<LotteryContract> {
        LotteryContract::instantiate(app, self, sender, name, label)
    }
}

impl From<LotteryCodeId> for u64 {
    fn from(code_id: LotteryCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct LotteryContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl LotteryContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: LotteryCodeId,
        sender: Addr,
        name: &str,

        label: &str,
    ) -> AnyResult<Self> {
        let init_msg = InstantiateMsg::new(name);

        app.instantiate_contract(
            code_id.0,
            Addr::unchecked(sender),
            &init_msg,
            &[],
            label,
            None,
        )
        .map(Self::from)
    }

    #[track_caller]
    pub fn buy(
        &self,
        app: &mut App,
        sender: Addr,
        lottery: &str,
        denom: &str,
        memo: Option<String>,
        funds: &[Coin],
    ) -> AnyResult<AppResponse> {
        app.execute_contract(
            sender,
            self.addr(),
            &ExecuteMsg::BuyTicket {
                lottery: lottery.into(),
                denom: denom.into(),
                memo,
            },
            funds,
        )
    }

    #[track_caller]
    pub fn draw_lottery(
        &self,
        app: &mut App,
        sender: Addr,
        lottery: &str,
    ) -> AnyResult<AppResponse> {
        app.execute_contract(
            sender,
            self.addr(),
            &ExecuteMsg::DrawLottery {
                lottery: lottery.into(),
            },
            &[],
        )
    }

    pub fn owner(&self, app: &App) -> StdResult<OwnerResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Owner {})
    }

    pub fn query_balances(app: &App, addr: Addr) -> StdResult<Vec<Coin>> {
        app.wrap().query_all_balances(addr)
    }

    pub fn query_state(&self, app: &App) -> StdResult<CurrentStateResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::CurrentState {})
    }

    pub fn players(&self, app: &App, address: &str) -> StdResult<PlayersResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Players {})
    }
}

impl From<Addr> for LotteryContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}

pub fn alice() -> Addr {
    Addr::unchecked("sei18rszd3tmgpjvjwq2qajtmn5jqvtscd2yuygl4z")
}

pub fn bob() -> Addr {
    Addr::unchecked("sei1aan9kqywf4rf274cal0hj6eyly6wu0uv7edxy2")
}

pub fn owner() -> Addr {
    Addr::unchecked("sei1zj6fjsc2gkce878ukzg6g9wy8cl8p554dlggxd")
}

pub fn parent() -> Addr {
    Addr::unchecked("inj1g9v8suckezwx93zypckd4xg03r26h6ejlmsptz")
}
