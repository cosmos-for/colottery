use std::fmt;
use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_storage_plus::{Item, Map};

use crate::{msg::PreparePrize, ContractError, Extension};

#[cw_serde]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub height: u64,
    pub created_at: Timestamp,
    pub expiratoin: Timestamp,
    pub unit_price: Coin,
    pub period: LotteryPeriod,
    pub selection: WinnerSelection,
    pub player_count: u64,
    pub max_players: u64,
    pub status: GameStatus,
    pub seed: String,
    pub winner: Vec<WinnerInfo>,
    pub category: LotteryCategory,
    pub extension: Extension,
}

impl State {
    pub fn is_closed(&self) -> bool {
        self.status == GameStatus::Closed
    }
}

#[cw_serde]
pub enum WinnerSelection {
    // Only a player win all prize
    Jackpot {},
    // Ex: [60, 30, 10] means 60% to 1st place, 30% to 2nd, 10% to 3rd
    Fixed {
        pct_split: Vec<u8>,
        winner_count: u32,
        max_winner_count: Option<u32>,
    },
}

impl WinnerSelection {
    pub fn is_jackpot(&self) -> bool {
        matches!(self, Self::Jackpot {})
    }
}

#[cw_serde]
pub enum GameStatus {
    Activing,
    Closed,
}

#[cw_serde]
pub enum LotteryCategory {
    Normal {},
    SpecifyPrize {},
}

impl Default for LotteryCategory {
    fn default() -> Self {
        Self::Normal {}
    }
}

impl FromStr for LotteryCategory {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls = s.trim().to_lowercase();

        match ls.as_str() {
            "normal" => Ok(Self::Normal {}),
            "specify_prize" => Ok(Self::SpecifyPrize {}),
            _ => Err(ContractError::InvalidCategory { value: s.into() }),
        }
    }
}

impl fmt::Display for LotteryCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal {} => write!(f, "Normal "),
            Self::SpecifyPrize {} => write!(f, "SpecifyPrize "),
        }
    }
}

#[cw_serde]
pub enum LotteryPeriod {
    Hour {},
    Day {},
    Week {},
    Month {},
    Year {},
}

impl Default for LotteryPeriod {
    fn default() -> Self {
        Self::Day {}
    }
}

impl FromStr for LotteryPeriod {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls = s.trim().to_lowercase();

        let period = match ls.as_str() {
            "hour" => Self::Hour {},
            "day" => Self::Day {},
            "week" => Self::Week {},
            "month" => Self::Month {},
            "year" => Self::Year {},
            _ => return Err(ContractError::InvalidLottoryPeriod { value: s.into() }),
        };

        Ok(period)
    }
}

impl LotteryPeriod {
    pub fn is_hour(&self) -> bool {
        matches!(self, Self::Hour {})
    }

    pub fn is_day(&self) -> bool {
        matches!(self, Self::Day {})
    }

    pub fn is_week(&self) -> bool {
        matches!(self, Self::Week {})
    }

    pub fn is_month(&self) -> bool {
        matches!(self, &Self::Month {})
    }

    pub fn is_year(&self) -> bool {
        matches!(self, &Self::Year {})
    }
}

#[cw_serde]
pub struct PlayerInfo {
    pub player_addr: Addr,
    pub lottery_addr: Addr,
    pub buy_at: u64,
    pub height: u64,
    pub ticket_id: String,
    pub memo: Option<String>,
}

#[cw_serde]
pub struct WinnerInfo {
    pub address: Addr,
    pub prize: Vec<Coin>,
    pub ticket_id: String,
}

/// Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const STATE: Item<State> = Item::new("state");
pub const PLAYERS: Map<&Addr, PlayerInfo> = Map::new("players");
pub const PLAYER_COUNTER: Item<u64> = Item::new("player_counter");
pub const IDX_2_ADDR: Map<u64, Addr> = Map::new("idx_2_addr");
pub const PREPARE_PRIZES: Map<u64, PreparePrize> = Map::new("prepare_prizes");

// pub const CLAIMS: Claims = Claims::new("claims");
#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lottery_period_should_works() {
        let hour = "hour";
        let day = "dAY";
        let week = "Week";
        let month = "moNth";
        let year = "yeaR";

        let period_h = hour.parse().unwrap();
        assert_eq!(LotteryPeriod::Hour {}, period_h);
        assert!(period_h.is_hour());

        let period_d = day.parse().unwrap();
        assert_eq!(LotteryPeriod::Day {}, period_d);
        assert!(period_d.is_day());

        let period_w = week.parse().unwrap();
        assert_eq!(LotteryPeriod::Week {}, period_w);
        assert!(period_w.is_week());

        let period_m = month.parse().unwrap();
        assert_eq!(LotteryPeriod::Month {}, period_m);
        assert!(period_m.is_month());

        let period_y = year.parse().unwrap();
        assert_eq!(LotteryPeriod::Year {}, period_y);
        assert!(period_y.is_year());
    }
}
