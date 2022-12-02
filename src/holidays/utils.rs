use chrono::{naive::NaiveDate, Days};
use computus::gregorian;

#[inline]
pub fn easter(year: i32) -> NaiveDate {
    let date = gregorian(year).unwrap();
    NaiveDate::from_ymd_opt(date.year, date.month, date.day).unwrap()
}

#[inline]
pub fn easter_monday(year: i32) -> NaiveDate {
    easter(year).checked_add_days(Days::new(1)).unwrap()
}

#[inline]
pub fn feast_of_the_ascension(year: i32) -> NaiveDate {
    easter(year).checked_add_days(Days::new(40)).unwrap()
}

#[inline]
pub fn whit_monday(year: i32) -> NaiveDate {
    easter(year).checked_add_days(Days::new(50)).unwrap()
}

#[inline]
pub fn feast_of_corpus_christi(year: i32) -> NaiveDate {
    easter(year).checked_add_days(Days::new(60)).unwrap()
}
