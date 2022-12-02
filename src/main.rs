use chrono::naive::NaiveDate;
use chrono::offset::Local;
use chrono::Datelike;
use clap::{value_parser, Arg, Command, Parser, ValueHint};
use how_many_days_until::count_days_in_between;

fn main() {
    let args = Command::new("how_many_days_until")
        .author("Domenic Melcher, domi.m@outlook.com")
        .version("0.1.0")
        .about("Command line tool to calculate how many days are until a date, considering national holidays.")
        .arg_required_else_help(true)
        .args([
            Arg::new("start")
                .short('s')
                .help("Start date; format: YYYY-MM-DD")
                .required(false)
                .value_hint(ValueHint::Other)
                .value_parser(value_parser!(NaiveDate)),
            Arg::new("end")
                .short('e')
                .help("Until when inclusive; format: YYYY-MM-DD")
                .required(true)
                .value_hint(ValueHint::Other)
                .value_parser(value_parser!(NaiveDate)),
        ])
        .get_matches();

    let start = match args.get_one::<NaiveDate>("start") {
        Some(date) => *date,
        None => Local::now().date_naive(),
    };
    let end = *args.get_one::<NaiveDate>("end").unwrap();

    let (total_days, working, free) = count_days_in_between(start, end);

    println!(
        "The {} is from {} in {} days.\nUntil then are {} working days and {} holidays",
        end, start, total_days, working, free
    );
}
