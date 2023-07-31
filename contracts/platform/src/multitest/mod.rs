#[cfg(test)]
mod tests;

use anyhow::Result as AnyResult;

use cosmwasm_std::{from_binary, Addr, Coin, StdResult};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};
use lottery::state::WinnerSelection;

use crate::{
    contract::{execute, instantiate, query, reply},
    msg::*,
};

#[derive(Clone, Debug, Copy)]
pub struct PlatformCodeId(u64);

impl PlatformCodeId {
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
        lottery_code_id: u64,
        label: &str,
    ) -> AnyResult<PlatformContract> {
        PlatformContract::instantiate(app, self, sender, name, lottery_code_id, label)
    }
}

impl From<PlatformCodeId> for u64 {
    fn from(code_id: PlatformCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct PlatformContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl PlatformContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: PlatformCodeId,
        sender: Addr,
        name: &str,
        lottery_code_id: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let init_msg = InstantiateMsg::new(name, lottery_code_id);

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

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn create_lottery(
        &self,
        app: &mut App,
        sender: Addr,
        name: &str,
        symbol: &str,
        unit_price_amount: u128,
        unit_price_denom: &str,
        period: &str,
        expiration: u64,
        selection: WinnerSelection,
        max_players: u64,
        label: &str,
    ) -> AnyResult<Option<InstantiationData>> {
        let msg = ExecuteMsg::CreateLottery {
            name: name.into(),
            symbol: symbol.into(),
            unit_price_amount,
            unit_price_denom: unit_price_denom.into(),
            period: period.into(),
            expiration,
            selection,
            max_players,
            label: label.into(),
        };

        let resp = app
            .execute_contract(sender, self.addr(), &msg, &[])
            .unwrap();

        // println!("execute create lottery resp:{:?}", resp);

        let data = from_binary(&resp.data.unwrap()).unwrap();

        Ok(data)
    }

    // #[track_caller]
    // pub fn buy_lottery(
    //     &self,
    //     app: &mut App,
    //     sender: Addr,
    //     lottery: &str,
    //     denom: &str,
    //     memo: Option<String>,
    //     funds: &[Coin],
    // ) -> AnyResult<AppResponse> {
    //     app.execute_contract(
    //         sender,
    //         self.addr(),
    //         &ExecuteMsg::BuyLottery {
    //             lottery: lottery.into(),
    //             denom: denom.into(),
    //             memo,
    //         },
    //         funds,
    //     )
    // }

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

    pub fn lotteries(&self, app: &App) -> StdResult<LotteriesResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Lotteries {})
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

    // pub fn players(&self, app: &App) -> StdResult<PlayersResp> {
    //     app.wrap()
    //         .query_wasm_smart(self.addr(), &QueryMsg::Players {})
    // }
}

impl From<Addr> for PlatformContract {
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
