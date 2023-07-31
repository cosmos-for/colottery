#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use chrono::Utc;
    use cosmwasm_std::{coin, coins, Empty};
    use cw_multi_test::App;

    use crate::{
        multitest::{alice, bob, owner, LotteryCodeId, LotteryContract, ARCH_DEMON},
        state::{GameStatus, WinnerSelection},
        ContractError,
    };

    use cw721_base::helpers::Cw721Contract;

    // use cw721_base::multi_tests;

    #[test]
    fn instantiate_should_works() {
        let mut app = App::default();
        let code_id = LotteryCodeId::store_code(&mut app);
        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price = 100;
        let denom = ARCH_DEMON;
        let period = "hour";
        let expiration = Utc::now().timestamp() as u64;
        let selection = WinnerSelection::Jackpot {};
        let max_players = 3;
        let label = "Lottery label";
        let contract = code_id
            .instantiate(
                &mut app,
                owner(),
                name,
                symbol,
                unit_price,
                denom,
                period,
                expiration,
                selection,
                max_players,
                label,
            )
            .unwrap();

        // check winner
        let winner = contract.winner(&app).unwrap();
        assert_eq!(winner.winner, vec![]);

        // check owner
        let contract_owner = contract.owner(&app).unwrap();
        assert_eq!(contract_owner.owner, owner());

        // check state
        let state = contract.query_state(&app).unwrap().state;
        assert_eq!(state.name, "LOTTERY");
        assert_eq!(state.unit_price, coin(100, ARCH_DEMON));
        assert_eq!(state.max_players, 3);
        assert_eq!(state.status, GameStatus::Activing);
        assert_eq!(state.player_count, 0);
        assert_eq!(state.selection, WinnerSelection::Jackpot {});

        // check is joined
        let is_joined = contract.player_info(&app, owner().as_str()).unwrap();
        assert!(is_joined.info.is_none());
    }

    #[test]
    fn lottery_full_flows_should_works() {
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &alice(), coins(300, ARCH_DEMON))
                .unwrap();
            router
                .bank
                .init_balance(storage, &bob(), coins(500, ARCH_DEMON))
                .unwrap();
        });

        let code_id = LotteryCodeId::store_code(&mut app);
        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price = 100;
        let denom = ARCH_DEMON;
        let period = "hour";
        let expiration = Utc::now().timestamp() as u64;
        let selection = WinnerSelection::Jackpot {};
        let max_players = 2;
        let label = "Lottery label";
        let contract = code_id
            .instantiate(
                &mut app,
                owner(),
                name,
                symbol,
                unit_price,
                denom,
                period,
                expiration,
                selection,
                max_players,
                label,
            )
            .unwrap();

        let cw721_contract: Cw721Contract<Empty, Empty> =
            Cw721Contract(contract.addr(), PhantomData, PhantomData);

        // Buy ticket
        contract
            .buy_ticket(
                &mut app,
                alice(),
                ARCH_DEMON,
                Some("恭喜发财!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        contract
            .buy_ticket(
                &mut app,
                bob(),
                ARCH_DEMON,
                Some("我要发".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        let nft_resp = cw721_contract.owner_of(&app.wrap(), "1", true).unwrap();
        assert_eq!(nft_resp.owner, alice());

        let nft_resp = cw721_contract.owner_of(&app.wrap(), "2", true).unwrap();
        assert_eq!(nft_resp.owner, bob());

        let balances = LotteryContract::query_balances(&app, contract.addr()).unwrap();
        assert_eq!(balances, coins(200, ARCH_DEMON));

        let balances = LotteryContract::query_balances(&app, alice()).unwrap();
        assert_eq!(balances, coins(200, ARCH_DEMON));

        let state = contract.query_state(&app).unwrap();
        assert_eq!(state.state.player_count, 2);

        let resp = contract
            .player_info(&app, alice().as_str())
            .unwrap()
            .info
            .unwrap();

        assert_eq!(resp.player_addr, alice());
        assert_eq!(resp.memo, Some("恭喜发财!".to_string()));

        // draw lottery
        contract.draw_lottery(&mut app, owner()).unwrap();

        contract.claim_lottery(&mut app, alice()).unwrap();

        let owner = contract.owner(&app).unwrap();
        assert_eq!(owner.owner, alice());

        let state = contract.query_state(&app).unwrap();
        assert_eq!(state.state.player_count, 2);
        assert_eq!(state.state.winner.len(), 1);

        let winner = state.state.winner.first().unwrap();
        assert_eq!(winner.address, alice());
        assert_eq!(winner.prize, coins(200, ARCH_DEMON));

        // withdraw funds
        contract
            .withdraw(&mut app, alice(), 100, ARCH_DEMON, None)
            .unwrap();
        contract
            .withdraw(&mut app, alice(), 100, ARCH_DEMON, Some(bob().to_string()))
            .unwrap();

        let balances = LotteryContract::query_balances(&app, contract.addr()).unwrap();
        assert!(balances.is_empty());

        let alice_balances = LotteryContract::query_balances(&app, alice()).unwrap();
        assert_eq!(alice_balances, coins(300, ARCH_DEMON));

        let bob_balances = LotteryContract::query_balances(&app, bob()).unwrap();
        assert_eq!(bob_balances, coins(500, ARCH_DEMON));
    }

    #[test]
    fn draw_lottery_should_fail() {
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &alice(), coins(300, ARCH_DEMON))
                .unwrap();
            router
                .bank
                .init_balance(storage, &bob(), coins(500, ARCH_DEMON))
                .unwrap();
        });

        let code_id = LotteryCodeId::store_code(&mut app);
        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price = 100;
        let denom = ARCH_DEMON;
        let period = "hour";
        let expiration = Utc::now().timestamp() as u64;
        let selection = WinnerSelection::Jackpot {};
        let max_players = 3;
        let label = "Lottery label";
        let contract = code_id
            .instantiate(
                &mut app,
                owner(),
                name,
                symbol,
                unit_price,
                denom,
                period,
                expiration,
                selection,
                max_players,
                label,
            )
            .unwrap();

        // Buy ticket
        contract
            .buy_ticket(
                &mut app,
                alice(),
                ARCH_DEMON,
                Some("恭喜发财!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        let err = contract.draw_lottery(&mut app, alice()).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, err.downcast().unwrap())
    }
}
