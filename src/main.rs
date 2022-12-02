use chrono::naive::NaiveDate;
use chrono::offset::Local;
use chrono::Datelike;
use count_days::count_days_in_between;

fn main() {
    let start = Local::now().date_naive();
    let end = NaiveDate::from_ymd_opt(start.year(), 12, 24).unwrap();

    let (total_days, working, free) = count_days_in_between(start, end);

    println!(
        "The {} is from {} in {} days.\nUntil then are {} working days and {} holidays",
        end, start, total_days, working, free
    );
}
