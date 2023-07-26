#[cfg(test)]
mod test {
    use cosmwasm_std::{coins, Uint128};
    use cw_multi_test::App;

    use crate::{
        multitest::{alice, bob, owner, LotteryCodeId, LotteryContract},
        state::{GameStatus, WinnerSelection},
        ContractError, ARCH_DEMON,
    };

    #[test]
    fn instantiate_should_works() {
        let mut app = App::default();
        let code_id = LotteryCodeId::store_code(&mut app);
        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price = 100;
        let period = "hour";
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
                period,
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
        assert_eq!(state.unit_price, Uint128::new(100));
        assert_eq!(state.max_players, 3);
        assert_eq!(state.status, GameStatus::Activing);
        assert_eq!(state.player_count, 0);
        assert_eq!(state.selection, WinnerSelection::Jackpot {});

        // check is joined
        let is_joined = contract.player_info(&app, owner().as_str()).unwrap();
        assert!(is_joined.info.is_none());
    }

    #[test]
    fn buy_lottery_should_works() {
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
        let period = "hour";
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
                period,
                selection,
                max_players,
                label,
            )
            .unwrap();

        // Buy ticket
        contract
            .buy(
                &mut app,
                alice(),
                ARCH_DEMON,
                Some("恭喜发财!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        contract
            .buy(
                &mut app,
                bob(),
                ARCH_DEMON,
                Some("我要发达!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

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
        let period = "hour";
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
                period,
                selection,
                max_players,
                label,
            )
            .unwrap();

        // Buy ticket
        contract
            .buy(
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

    #[test]
    fn withdraw_lottery_should_works() {
        // let mut app = App::new(|router, _api, storage| {
        //     router
        //         .bank
        //         .init_balance(storage, &owner(), coins(3000, NATIVE_DENOM))
        //         .unwrap();
        //     router
        //         .bank
        //         .init_balance(storage, &alice(), coins(1000, NATIVE_DENOM))
        //         .unwrap();
        // });

        // let code_id = LotteryCodeId::store_code(&mut app);
        // let title = "lottery title";
        // let contract = code_id
        //     .instantiate(&mut app, owner(), title, "lottery test")
        //     .unwrap();

        // contract
        //     .buy(
        //         &mut app,
        //         alice(),
        //         NATIVE_DENOM,
        //         Some("恭喜发财!".to_string()),
        //         &coins(100, NATIVE_DENOM),
        //     )
        //     .unwrap();
        // let resp = contract.bettor_count(&app, alice().as_str()).unwrap();
        // let expected = BetInfo {
        //     buy_at: 12345,
        //     memo: Some("恭喜发财!".to_string()),
        // };
        // assert_eq!(resp.info, Some(expected));

        // contract
        //     .draw(&mut app, owner(), &coins(1000, NATIVE_DENOM))
        //     .unwrap();
        // contract
        //     .withdraw(&mut app, alice(), 1000, NATIVE_DENOM)
        //     .unwrap();

        // let lottery_balance = LotteryContract::query_balances(&app, contract.addr()).unwrap();
        // assert_eq!(lottery_balance, coins(100, NATIVE_DENOM));

        // let alice_balance = LotteryContract::query_balances(&app, alice()).unwrap();
        // assert_eq!(alice_balance, coins(1900, NATIVE_DENOM));

        // let owner_balance = LotteryContract::query_balances(&app, owner()).unwrap();
        // assert_eq!(owner_balance, coins(2000, NATIVE_DENOM));
    }
}
