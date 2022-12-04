use chrono::naive::NaiveDate;
use chrono::offset::Local;
use clap::{value_parser, Arg, ArgMatches, Command, ValueHint};
use how_many_days_until::count_days_in_between;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Args {
    start: NaiveDate,
    end: NaiveDate,
}

fn app() -> Command {
    Command::new("how_many_days_until")
        .author("Domenic Melcher, domi.m@outlook.com")
        .version(VERSION)
        .about("Command line tool to calculate how many days are until a date, considering national holidays.")
        .arg_required_else_help(true)
        .args([
            Arg::new("start")
                .short('s')
                .help("Start date; format: YYYY-MM-DD; default: today")
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
}

fn parse_matches(args: ArgMatches) -> Args {
    let start = match args.get_one::<NaiveDate>("start") {
        Some(date) => *date,
        None => Local::now().date_naive(),
    };
    let end = *args.get_one::<NaiveDate>("end").unwrap();

    Args { start, end }
}

fn main() {
    let matches = app().get_matches();
    let args = parse_matches(matches);

    let (total_days, working, free) = count_days_in_between(args.start, args.end);

    println!(
        "The {} is from {} in {} days.\nUntil then are {} working days and {} holidays",
        args.end, args.start, total_days, working, free
    );
}
