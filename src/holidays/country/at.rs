use chrono::naive::NaiveDate;

use crate::holidays::utils::{
    easter_monday, feast_of_corpus_christi, feast_of_the_ascension, whit_monday,
};
use crate::holidays::HolidaysAsListPerYear;

/// Austria
pub struct AT {}

impl AT {
    pub fn new_year(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap()
    }

    pub fn biblical_magi(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 1, 6).unwrap()
    }

    pub fn easter(year: i32) -> NaiveDate {
        easter_monday(year)
    }

    pub fn country_holiday(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 5, 1).unwrap()
    }

    pub fn feast_of_the_ascension(year: i32) -> NaiveDate {
        feast_of_the_ascension(year)
    }

    pub fn whit_monday(year: i32) -> NaiveDate {
        whit_monday(year)
    }

    pub fn feast_of_corpus_christi(year: i32) -> NaiveDate {
        feast_of_corpus_christi(year)
    }

    pub fn assumption_of_mary(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 8, 15).unwrap()
    }

    pub fn national_holiday(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 10, 26).unwrap()
    }

    pub fn all_saints_day(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 11, 1).unwrap()
    }

    pub fn immaculate_conception_day(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 12, 8).unwrap()
    }

    pub fn christmas(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 12, 25).unwrap()
    }

    pub fn saint_stephen_day(year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, 12, 26).unwrap()
    }
}

impl HolidaysAsListPerYear for AT {
    // TODO replace with macro that works on `impl AT {...}` and generates this function automaticly
    fn as_list(year: i32) -> Vec<NaiveDate> {
        let functions = vec![
            AT::new_year as fn(i32) -> NaiveDate,
            AT::country_holiday,
            AT::easter,
            AT::feast_of_the_ascension,
            AT::whit_monday,
            AT::feast_of_corpus_christi,
            AT::national_holiday,
            AT::all_saints_day,
            AT::assumption_of_mary,
            AT::biblical_magi,
            AT::immaculate_conception_day,
            AT::christmas,
            AT::saint_stephen_day,
        ];
        functions.iter().map(|f| f(year)).collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::naive::NaiveDate;

    use super::AT;

    #[test]
    fn year_2022() {
        assert_eq!(
            AT::new_year(2023),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
        assert_eq!(
            AT::biblical_magi(2023),
            NaiveDate::from_ymd_opt(2023, 1, 6).unwrap()
        );
        assert_eq!(
            AT::easter(2023),
            NaiveDate::from_ymd_opt(2023, 4, 10).unwrap()
        );
        assert_eq!(
            AT::country_holiday(2023),
            NaiveDate::from_ymd_opt(2023, 5, 1).unwrap()
        );
        // TODO why?
        // assert_eq!(
        //     AT::assumption_of_mary(2023),
        //     NaiveDate::from_ymd_opt(2023, 5, 18).unwrap()
        // );
        assert_eq!(
            AT::whit_monday(2023),
            NaiveDate::from_ymd_opt(2023, 5, 29).unwrap()
        );
        assert_eq!(
            AT::feast_of_corpus_christi(2023),
            NaiveDate::from_ymd_opt(2023, 6, 8).unwrap()
        );
        assert_eq!(
            AT::assumption_of_mary(2023),
            NaiveDate::from_ymd_opt(2023, 8, 15).unwrap()
        );
        assert_eq!(
            AT::national_holiday(2023),
            NaiveDate::from_ymd_opt(2023, 10, 26).unwrap()
        );
        assert_eq!(
            AT::all_saints_day(2023),
            NaiveDate::from_ymd_opt(2023, 11, 1).unwrap()
        );
        assert_eq!(
            AT::immaculate_conception_day(2023),
            NaiveDate::from_ymd_opt(2023, 12, 8).unwrap()
        );
        assert_eq!(
            AT::christmas(2023),
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(
            AT::saint_stephen_day(2023),
            NaiveDate::from_ymd_opt(2023, 12, 26).unwrap()
        );
    }

    #[test]
    fn year_2023() {
        assert_eq!(
            AT::new_year(2023),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
        assert_eq!(
            AT::biblical_magi(2023),
            NaiveDate::from_ymd_opt(2023, 1, 6).unwrap()
        );
        assert_eq!(
            AT::easter(2023),
            NaiveDate::from_ymd_opt(2023, 4, 10).unwrap()
        );
        assert_eq!(
            AT::country_holiday(2023),
            NaiveDate::from_ymd_opt(2023, 5, 1).unwrap()
        );
        // TODO why?
        // assert_eq!(
        //     AT::assumption_of_mary(2023),
        //     NaiveDate::from_ymd_opt(2023, 5, 18).unwrap()
        // );
        assert_eq!(
            AT::whit_monday(2023),
            NaiveDate::from_ymd_opt(2023, 5, 29).unwrap()
        );
        assert_eq!(
            AT::feast_of_corpus_christi(2023),
            NaiveDate::from_ymd_opt(2023, 6, 8).unwrap()
        );
        assert_eq!(
            AT::assumption_of_mary(2023),
            NaiveDate::from_ymd_opt(2023, 8, 15).unwrap()
        );
        assert_eq!(
            AT::national_holiday(2023),
            NaiveDate::from_ymd_opt(2023, 10, 26).unwrap()
        );
        assert_eq!(
            AT::all_saints_day(2023),
            NaiveDate::from_ymd_opt(2023, 11, 1).unwrap()
        );
        assert_eq!(
            AT::immaculate_conception_day(2023),
            NaiveDate::from_ymd_opt(2023, 12, 8).unwrap()
        );
        assert_eq!(
            AT::christmas(2023),
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(
            AT::saint_stephen_day(2023),
            NaiveDate::from_ymd_opt(2023, 12, 26).unwrap()
        );
    }

    #[test]
    fn year_2024() {
        assert_eq!(
            AT::new_year(2024),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(
            AT::biblical_magi(2024),
            NaiveDate::from_ymd_opt(2024, 1, 6).unwrap()
        );
        assert_eq!(
            AT::easter(2024),
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()
        );
        assert_eq!(
            AT::country_holiday(2024),
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
        );
        // TODO why?
        // assert_eq!(
        //     AT::assumption_of_mary(2024),
        //     NaiveDate::from_ymd_opt(2024, 5, 9).unwrap()
        // );
        assert_eq!(
            AT::whit_monday(2024),
            NaiveDate::from_ymd_opt(2024, 5, 20).unwrap()
        );
        assert_eq!(
            AT::feast_of_corpus_christi(2024),
            NaiveDate::from_ymd_opt(2024, 5, 30).unwrap()
        );
        assert_eq!(
            AT::assumption_of_mary(2024),
            NaiveDate::from_ymd_opt(2024, 8, 15).unwrap()
        );
        assert_eq!(
            AT::national_holiday(2024),
            NaiveDate::from_ymd_opt(2024, 10, 26).unwrap()
        );
        assert_eq!(
            AT::all_saints_day(2024),
            NaiveDate::from_ymd_opt(2024, 11, 1).unwrap()
        );
        assert_eq!(
            AT::immaculate_conception_day(2024),
            NaiveDate::from_ymd_opt(2024, 12, 8).unwrap()
        );
        assert_eq!(
            AT::christmas(2024),
            NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()
        );
        assert_eq!(
            AT::saint_stephen_day(2024),
            NaiveDate::from_ymd_opt(2024, 12, 26).unwrap()
        );
    }
}
