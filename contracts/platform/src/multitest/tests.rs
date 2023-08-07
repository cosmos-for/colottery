#[cfg(test)]
mod test {
    use chrono::Utc;
    use cosmwasm_std::{coin, coins};
    use cw_multi_test::App;
    use lottery::{
        multitest::{LotteryCodeId, LotteryContract},
        state::{LotteryPeriod, WinnerSelection},
    };

    use crate::{
        multitest::{alice, bob, owner, PlatformCodeId, PlatformContract},
        ContractError, ARCH_DEMON,
    };

    #[test]
    fn platform_instantiate_should_works() {
        let mut app = App::default();
        let code_id = PlatformCodeId::store_code(&mut app);
        let lottery_code_id = LotteryCodeId::store_code(&mut app);
        let name = "PLATFORM";
        let label = "Lottery label";
        let contract = code_id
            .instantiate(&mut app, owner(), name, lottery_code_id.into(), label)
            .unwrap();

        // check owner
        let contract_owner = contract.owner(&app).unwrap();
        assert_eq!(contract_owner.owner, owner());

        // check state
        let state = contract.query_state(&app).unwrap().state;
        assert_eq!(state.name, "PLATFORM");
        assert_eq!(state.lotteries_count, 0);
        // assert_eq!(state.players_count, 0);

        // check balances
        let balances = PlatformContract::query_balances(&app, contract.addr()).unwrap();
        assert!(balances.is_empty());
    }

    #[test]
    fn platform_create_lottery_should_works() {
        let mut app = App::default();
        let code_id = PlatformCodeId::store_code(&mut app);
        let lottery_code_id = LotteryCodeId::store_code(&mut app);
        let name = "PLATFORM";
        let label = "Lottery label";
        let contract = code_id
            .instantiate(&mut app, owner(), name, lottery_code_id.into(), label)
            .unwrap();

        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price_amount = 100;
        let unit_price_denom = ARCH_DEMON;
        let period = "hour";
        let expiration = Utc::now().timestamp() as u64;
        let selection = WinnerSelection::Jackpot {};
        let max_players = 3;
        let label = "Lottery label";

        let resp = contract
            .create_lottery(
                &mut app,
                owner(),
                name,
                symbol,
                unit_price_amount,
                unit_price_denom,
                period,
                expiration,
                selection,
                max_players,
                None,
                label,
            )
            .unwrap();

        println!("create lottery resp:{:?}", resp);

        let lottery_addr = resp.unwrap().addr;

        let state = contract.query_state(&app).unwrap().state;
        assert_eq!(state.lotteries_count, 1);

        let lotteries = contract.lotteries(&app).unwrap();
        assert_eq!(lotteries.lotteries.len(), 1);

        let lottery = &lotteries.lotteries[0];
        assert_eq!(lottery.name, name);
        assert_eq!(lottery.symbol, symbol);
        assert_eq!(
            lottery.unit_price,
            coin(unit_price_amount, unit_price_denom)
        );
        assert_eq!(lottery.period, LotteryPeriod::Hour {});
        assert_eq!(lottery.selection, WinnerSelection::Jackpot {});
        assert_eq!(lottery.max_players, max_players);
        assert_eq!(lottery.contract_addr, lottery_addr);
    }

    #[test]
    fn platform_buy_and_draw_lottery_should_works() {
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

        let code_id = PlatformCodeId::store_code(&mut app);
        let lottery_code_id = LotteryCodeId::store_code(&mut app);
        let name = "PLATFORM";
        let label = "Lottery label";
        let contract = code_id
            .instantiate(&mut app, owner(), name, lottery_code_id.into(), label)
            .unwrap();

        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price_amount = 100;
        let unit_price_denom = ARCH_DEMON;
        let period = "hour";
        let expiration = Utc::now().timestamp() as u64;
        let selection = WinnerSelection::Jackpot {};
        let max_players = 2;
        let label = "Lottery label";

        let resp = contract
            .create_lottery(
                &mut app,
                owner(),
                name,
                symbol,
                unit_price_amount,
                unit_price_denom,
                period,
                expiration,
                selection,
                max_players,
                None,
                label,
            )
            .unwrap();

        println!("create lottery resp:{:?}", resp);

        let lottery_addr = &resp.unwrap().addr;
        let lottery_contract: LotteryContract = lottery_addr.to_owned().into();

        // Buy ticket
        lottery_contract
            .buy_ticket(
                &mut app,
                alice(),
                ARCH_DEMON,
                Some("恭喜发财!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        lottery_contract
            .buy_ticket(
                &mut app,
                bob(),
                ARCH_DEMON,
                Some("我要发".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        let balances = PlatformContract::query_balances(&app, lottery_addr.to_owned()).unwrap();
        assert_eq!(balances, coins(200, ARCH_DEMON));

        let balances = LotteryContract::query_balances(&app, alice()).unwrap();
        assert_eq!(balances, coins(200, ARCH_DEMON));

        let state = lottery_contract.query_state(&app).unwrap();
        assert_eq!(state.state.player_count, 2);

        let resp = lottery_contract
            .player_info(&app, alice().as_str())
            .unwrap()
            .info
            .unwrap();

        assert_eq!(resp.player_addr, alice());
        assert_eq!(resp.memo, Some("恭喜发财!".to_string()));

        // check lottery owner
        let lottery_owner = lottery_contract.owner(&app).unwrap();
        assert_eq!(lottery_owner.owner, contract.addr());

        // draw lottery
        contract
            .draw_lottery(&mut app, owner(), lottery_addr.as_str())
            .unwrap();

        // check lottery winner
        let winner = lottery_contract.winner(&app).unwrap();
        assert_eq!(winner.winner[0].address, alice());
        assert_eq!(winner.winner[0].prize, coins(200, ARCH_DEMON));

        lottery_contract.claim_lottery(&mut app, alice()).unwrap();

        let owner = lottery_contract.owner(&app).unwrap();
        assert_eq!(owner.owner, alice());

        let state = lottery_contract.query_state(&app).unwrap();
        assert_eq!(state.state.player_count, 2);
        assert_eq!(state.state.winner.len(), 1);

        let winner = state.state.winner.first().unwrap();
        assert_eq!(winner.address, alice());
        assert_eq!(winner.prize, coins(200, ARCH_DEMON));

        // withdraw funds
        lottery_contract
            .withdraw(&mut app, alice(), 100, ARCH_DEMON, None)
            .unwrap();
        lottery_contract
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

        let code_id = PlatformCodeId::store_code(&mut app);
        let lottery_code_id = LotteryCodeId::store_code(&mut app);
        let name = "PLATFORM";
        let label = "Lottery label";
        let contract = code_id
            .instantiate(&mut app, owner(), name, lottery_code_id.into(), label)
            .unwrap();

        let name = "LOTTERY";
        let symbol = "LOTTER";
        let unit_price_amount = 100;
        let unit_price_denom = ARCH_DEMON;
        let period = "hour";
        let expiration = Utc::now().timestamp() as u64;
        let selection = WinnerSelection::Jackpot {};
        let max_players = 3;
        let label = "Lottery label";

        let resp = contract.create_lottery(
            &mut app,
            owner(),
            name,
            symbol,
            unit_price_amount,
            unit_price_denom,
            period,
            expiration,
            selection,
            max_players,
            None,
            label,
        );

        println!("create lottery resp:{:?}", resp);

        let lotteries = contract.lotteries(&app).unwrap();
        let lottery = &lotteries.lotteries[0];
        let lottery_addr = &lottery.contract_addr;
        let lottery_contract: LotteryContract = lottery_addr.to_owned().into();

        // Buy ticket
        lottery_contract
            .buy_ticket(
                &mut app,
                alice(),
                ARCH_DEMON,
                Some("恭喜发财!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        lottery_contract
            .buy_ticket(
                &mut app,
                bob(),
                ARCH_DEMON,
                Some("我要发达!".to_string()),
                &coins(100, ARCH_DEMON),
            )
            .unwrap();

        // draw lottery
        let err = contract
            .draw_lottery(&mut app, alice(), lottery_addr.as_str())
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, err.downcast().unwrap())
    }
}
