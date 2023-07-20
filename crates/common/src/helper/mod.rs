use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc, Weekday};
use cosmwasm_std::Timestamp;

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

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
