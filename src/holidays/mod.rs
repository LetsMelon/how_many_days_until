use chrono::NaiveDate;

pub mod country;
mod utils;

pub enum CountryCode {
    /// Austria
    AT,
}

pub trait HolidaysAsListPerYear {
    fn as_list(year: i32) -> Vec<NaiveDate>;
}

pub fn get_country(country: CountryCode, year: i32) -> Vec<NaiveDate> {
    match country {
        CountryCode::AT => country::AT::as_list(year),
    }
}
