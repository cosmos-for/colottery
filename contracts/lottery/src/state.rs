use std::str::FromStr;

use common::helper::{
    get_last_day_month, get_last_day_week, get_last_day_year, get_secs_of_hour_22, timestamp_to_utc,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

use crate::{ContractError, Extension};

#[cw_serde]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub created_at: Timestamp,
    pub expiratoin: Timestamp,
    pub unit_price: Uint128,
    pub period: LotteryPeriod,
    pub selection: WinnerSelection,
    pub player_count: u32,
    pub max_players: u32,
    pub status: GameStatus,
    pub winner: Vec<WinnerInfo>,
    pub extension: Extension,
}

#[cw_serde]
pub enum WinnerSelection {
    // Only a player win all prize
    OnlyOne {},
    // Ex: [60, 30, 10] means 60% to 1st place, 30% to 2nd, 10% to 3rd
    Fixed {
        pct_split: Vec<u8>,
        winner_count: u32,
        max_winner_count: Option<u32>,
    },
}

#[cw_serde]
pub enum GameStatus {
    Activing,
    Ended,
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
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
        let ls = s.to_lowercase();
        let ls = ls.as_str();
        let period = match ls {
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

    pub fn get_deadline(&self, created_at: Timestamp) -> Timestamp {
        match self {
            Self::Hour {} => created_at.plus_hours(1),
            Self::Day {} => Timestamp::from_seconds(get_secs_of_hour_22(
                timestamp_to_utc(created_at).date_naive(),
            )),
            Self::Week {} => Timestamp::from_seconds(get_secs_of_hour_22(get_last_day_week(
                timestamp_to_utc(created_at),
            ))),
            Self::Month {} => Timestamp::from_seconds(get_secs_of_hour_22(get_last_day_month(
                timestamp_to_utc(created_at),
            ))),
            Self::Year {} => Timestamp::from_seconds(get_secs_of_hour_22(get_last_day_year(
                timestamp_to_utc(created_at),
            ))),
        }
    }
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

#[cw_serde]
pub struct PlayerInfo {
    pub address: Addr,
    pub buy_at: u64,
    pub height: u64,
    pub memo: Option<String>,
}

#[cw_serde]
pub struct WinnerInfo {
    pub address: Addr,
    pub prize: Coin,
}

/// Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const STATE: Item<State> = Item::new("state");
pub const PLAYERS: Map<&Addr, PlayerInfo> = Map::new("players");

// pub const CLAIMS: Claims = Claims::new("claims");

#[cfg(test)]
mod tests {
    use chrono::Utc;

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

    #[test]
    fn get_deadline_should_works() {
        let day = LotteryPeriod::Day {};
        let utc_now = Utc::now();
        println!("utc now is: {:?}", utc_now.timestamp());

        let utc_22 = utc_now.date_naive().and_hms_opt(22, 0, 0);
        let now_secs = utc_22.map(|t| t.timestamp()).unwrap() as u64;
        println!("utc sces is: {:?}", now_secs);

        let now = Timestamp::from_seconds(now_secs);

        println!("bft now seconds: {}", now.seconds());

        assert_eq!(now_secs, now.seconds());

        let deadline = day.get_deadline(now);
        println!("deadline is: {:?}", deadline.seconds());
        assert_eq!(deadline.seconds(), now.seconds());
    }
}
