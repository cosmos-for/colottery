use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

use crate::{
    msg::InstantiateMsg,
    state::{GameStatus, LotteryPeriod, State, OWNER, STATE},
    ContractError, Cw721MetadataContract,
};

use super::{CONTRACT_NAME, CONTRACT_VERSION};

pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.unit_price == Uint128::zero() {
        return Err(ContractError::InvalidUnitPrice {
            value: msg.unit_price,
        });
    }

    // Only support OnlyOnce now
    if !msg.selection.is_jackpot() {
        return Err(ContractError::UnSupportedWinnerSelection {
            selection: msg.selection,
        });
    }

    let created_at = env.block.time;
    let period: LotteryPeriod = msg.period.parse()?;
    let expiration = period.get_deadline(created_at);

    let config = State {
        name: msg.name.clone(),
        symbol: msg.symobl.clone(),
        height: env.block.height,
        created_at,
        expiratoin: expiration,
        unit_price: msg.unit_price,
        period,
        selection: msg.selection,
        player_count: 0,
        max_players: msg.max_bettors,
        status: GameStatus::Activing,
        winner: vec![],
        extension: Default::default(),
    };

    STATE.save(deps.storage, &config)?;
    OWNER.save(deps.storage, &info.sender)?;

    let init_msg = Cw721InstantiateMsg {
        name: msg.name,
        symbol: msg.symobl,
        minter: info.sender.to_string(),
    };

    Cw721MetadataContract::default().instantiate(deps.branch(), env, info, init_msg)?;

    Ok(Response::new())
}
