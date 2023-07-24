#[cfg(test)]
mod test {
    use cosmwasm_std::Uint128;
    use cw_multi_test::App;

    use crate::{
        multitest::{owner, LotteryCodeId},
        state::{GameStatus, WinnerSelection},
    };

    #[test]
    fn instantiate_should_works() {
        let mut app = App::default();
        let code_id = LotteryCodeId::store_code(&mut app);
        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price = 100;
        let period = "hour";
        let selection = WinnerSelection::OnlyOne {};
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
        assert_eq!(state.selection, WinnerSelection::OnlyOne {});

        // check is joined
        let is_joined = contract.is_joined(&app, owner().as_str()).unwrap();
        assert!(!is_joined.joined);
    }

    #[test]
    fn buy_lottery_should_works() {
        // let mut app = App::new(|router, _api, storage| {
        //     router
        //         .bank
        //         .init_balance(storage, &alice(), coins(3000, NATIVE_DENOM))
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
    }

    #[test]
    fn draw_lottery_should_fail() {
        // let mut app = App::new(|router, _api, storage| {
        //     router
        //         .bank
        //         .init_balance(storage, &alice(), coins(3000, NATIVE_DENOM))
        //         .unwrap();
        // });

        // let code_id = LotteryCodeId::store_code(&mut app);
        // let title = "lottery title";
        // let contract = code_id
        //     .instantiate(&mut app, owner(), title, "lottery test")
        //     .unwrap();

        // let err = contract
        //     .draw(&mut app, alice(), &coins(1000, NATIVE_DENOM))
        //     .unwrap_err();
        // assert_eq!(ContractError::UnauthorizedErr {}, err.downcast().unwrap())
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
