#[cfg(test)]
mod test {
    use cosmwasm_std::coins;
    use cw_multi_test::App;

    use crate::{
        multitest::{alice, owner, LotteryCodeId, LotteryContract},
        state::BetInfo,
        ContractError, NATIVE_DENOM,
    };

    #[test]
    fn instantiate_should_works() {
        let mut app = App::default();
        let code_id = LotteryCodeId::store_code(&mut app);
        let title = "lottery title";
        let contract = code_id
            .instantiate(&mut app, owner(), title, "lottery test")
            .unwrap();

        let winner = contract.winner(&app).unwrap();
        assert_eq!(winner.winner, None);

        // let contract_owner = contract.owner(&app).unwrap();
        // assert_eq!(contract_owner.owner, owner());
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
