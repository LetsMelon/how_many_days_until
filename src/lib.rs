use chrono::naive::NaiveDate;
use chrono::{Datelike, Weekday};

pub mod holidays;

use holidays::{get_country, CountryCode};

pub fn is_weekday(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    match weekday {
        Weekday::Sat | Weekday::Sun => false,
        _ => true,
    }
}

pub fn is_weekend(date: NaiveDate) -> bool {
    !is_weekday(date)
}

pub fn is_holiday(date: NaiveDate, country: CountryCode) -> bool {
    let holidays = get_country(country, date.year());
    holidays.contains(&date)
}

pub fn days_in_range(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let days = NaiveDate::signed_duration_since(end, start).num_days();
    if days < 0 {
        Vec::new()
    } else {
        start.iter_days().take(days as usize + 1).collect()
    }
}

pub fn count_days_in_between(start: NaiveDate, end: NaiveDate) -> (usize, usize, usize) {
    let days_in_between = days_in_range(start, end);
    let only_weekdays = days_in_between
        .iter()
        .cloned()
        .filter(|date| is_weekday(*date) && !is_holiday(*date, CountryCode::AT))
        .count();
    let only_weekend_or_holiday = days_in_between
        .iter()
        .cloned()
        .filter(|date| is_weekend(*date) || is_holiday(*date, CountryCode::AT))
        .count();

    (
        days_in_between.len(),
        only_weekdays,
        only_weekend_or_holiday,
    )
}
