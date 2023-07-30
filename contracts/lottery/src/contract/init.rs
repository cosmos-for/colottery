use common::hash;
use cosmwasm_std::{attr, coin, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::{
    msg::InstantiateMsg,
    state::{GameStatus, LotteryPeriod, State, OWNER, PLAYER_COUNTER, STATE},
    ContractError, Cw721InstantiateMsg, Cw721MetadataContract,
};

use super::{CONTRACT_NAME, CONTRACT_VERSION};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.unit_price_amount == 0 {
        return Err(ContractError::InvalidUnitPrice {
            value: msg.unit_price_amount,
        });
    }

    // Only support OnlyOnce now
    if !msg.selection.is_jackpot() {
        return Err(ContractError::UnSupportedWinnerSelection {
            selection: msg.selection,
        });
    }

    let sender = &info.sender;
    let created_at = env.block.time;
    let period: LotteryPeriod = msg.period.parse()?;
    let expiration = period.get_deadline(created_at);

    let config = State {
        name: msg.name.clone(),
        symbol: msg.symobl.clone(),
        height: env.block.height,
        created_at,
        expiratoin: expiration,
        unit_price: coin(msg.unit_price_amount, msg.unit_price_denom),
        period,
        selection: msg.selection,
        player_count: 0,
        max_players: msg.max_players,
        status: GameStatus::Activing,
        seed: hash::seed::init(env.contract.address.as_str(), env.block.height),
        winner: vec![],
        extension: Default::default(),
    };

    STATE.save(deps.storage, &config)?;
    OWNER.save(deps.storage, sender)?;
    PLAYER_COUNTER.save(deps.storage, &0)?;

    let init_msg = Cw721InstantiateMsg {
        name: msg.name,
        symbol: msg.symobl,
        minter: info.sender.to_string(),
    };

    let attrs = vec![
        attr("action", "instantiate"),
        attr("sender", sender.as_str()),
    ];

    let cw721_contract: Cw721MetadataContract = Cw721MetadataContract::default();
    cw721_contract.instantiate(deps, env, info, init_msg)?;

    Ok(Response::new().add_attributes(attrs))
}
