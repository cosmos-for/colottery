use std::str::FromStr;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::Item;

use crate::{ContractError, Extension};

#[cw_serde]
pub struct Config {
    pub name: String,
    pub symbol: String,
    pub created_at: Timestamp,
    pub deadline: Timestamp,
    pub unit_price: Uint128,
    pub period: LotteryPeriod,
    pub winner: Option<String>,
    pub extension: Extension,
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

pub fn timestamp_to_utc(ts: Timestamp) -> DateTime<Utc> {
    // Get seconds from created_at
    let seconds = ts.seconds();
    // Transfer secons to DateTime<Utc>
    Utc.timestamp_opt(seconds as i64, 0).unwrap()
}

pub fn get_last_day_week(dt: DateTime<Utc>) -> NaiveDate {
    let date = dt.date_naive();
    let week = date.week(Weekday::Sun);
    week.last_day()
}

pub fn get_last_day_month(dt: DateTime<Utc>) -> NaiveDate {
    // Get the year, month, week of the datetime
    let (year, month) = (dt.year(), dt.month());
    // calculate the last day of the month
    let next_month_first_day = Utc
        .with_ymd_and_hms(year + (month as i32 / 12), (month % 12) + 1, 1, 22, 0, 0)
        .unwrap();
    (next_month_first_day - chrono::Duration::days(1)).date_naive()
}

pub fn get_last_day_year(dt: DateTime<Utc>) -> NaiveDate {
    // Get the year, month, week of the datetime
    let year = dt.year();
    // calculate the last day of the month
    let next_month_first_day = Utc.with_ymd_and_hms(year + 1, 1, 1, 22, 0, 0).unwrap();
    (next_month_first_day - chrono::Duration::days(1)).date_naive()
}

pub fn get_secs_of_hour_22(nd: NaiveDate) -> u64 {
    let dt_22 = nd.and_hms_opt(22, 0, 0).unwrap();
    dt_22.timestamp() as u64
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

/// Storage
pub const OWNER: Item<Addr> = Item::new("owner");
pub const CONIFG: Item<Config> = Item::new("config");

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
        let hour = LotteryPeriod::Hour {};
        let utc_now = Utc::now();
        println!("utc now is: {:?}", utc_now);

        let utc_22 = utc_now.date_naive().and_hms_opt(22, 0, 0);
        let now_secs = utc_22.map(|t| t.timestamp()).unwrap() as u64;
        println!("utc nanos is: {:?}", now_secs);

        let now = Timestamp::from_seconds(now_secs);

        println!("bft now seconds: {}", now.seconds());

        assert_eq!(now_secs, now.seconds());
    }

    #[test]
    fn get_last_day_year_should_works() {
        let dt = Utc::now();

        let last_day = get_last_day_year(dt);

        assert_eq!(last_day.month(), 12);
        assert_eq!(last_day.year(), dt.year());
        assert_eq!(last_day.day(), 31);

        let dt = Utc.with_ymd_and_hms(2023, 2, 20, 0, 0, 0).unwrap();

        let last_day = get_last_day_year(dt);

        assert_eq!(last_day.month(), 12);
        assert_eq!(last_day.year(), 2023);
        assert_eq!(last_day.day(), 31);

        let dt = Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap();

        let last_day = get_last_day_year(dt);

        assert_eq!(last_day.month(), 12);
        assert_eq!(last_day.year(), dt.year());
        assert_eq!(last_day.day(), 31);
    }

    #[test]
    fn get_last_day_month_should_works() {
        // for Febuary
        let dt = Utc.with_ymd_and_hms(2023, 2, 20, 0, 0, 0).unwrap();

        let last_day = get_last_day_month(dt);

        assert_eq!(last_day.month(), dt.month());
        assert_eq!(last_day.year(), dt.year());
        assert_eq!(last_day.day(), 28);

        // for August
        let dt = Utc.with_ymd_and_hms(2023, 8, 20, 0, 0, 0).unwrap();

        let last_day = get_last_day_month(dt);

        assert_eq!(last_day.month(), dt.month());
        assert_eq!(last_day.year(), dt.year());
        assert_eq!(last_day.day(), 31);

        // for December
        let dt = Utc.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap();

        let last_day = get_last_day_month(dt);

        assert_eq!(last_day.month(), dt.month());
        assert_eq!(last_day.year(), dt.year());
        assert_eq!(last_day.day(), 31);
    }
}
