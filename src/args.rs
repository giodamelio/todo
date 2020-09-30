use chrono::{NaiveDate, NaiveTime};
use clap::{crate_authors, crate_description, crate_version, AppSettings, Clap};
use date_time_parser::{DateParser, TimeParser};
use log::LevelFilter;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!("\n"), about = crate_description!())]
pub struct Args {
    #[clap(
        long,
        env = "RUST_LOG",
        default_value = "info",
        about = "Set the log level"
    )]
    pub log_level: LevelFilter,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    #[clap(about = "Add a TODO", setting = AppSettings::TrailingVarArg)]
    Add(Add),
}

#[derive(Clap, Debug)]
pub struct Add {
    #[clap(short, long, parse(try_from_str = parse_natural_datetime), about = "The datetime that the TODO is due")]
    due: Option<TimeOrDate>,

    #[clap(multiple = true)]
    message: Vec<String>,
}

#[derive(Debug)]
pub enum TimeOrDate {
    Time(NaiveTime),
    Date(NaiveDate),
}

// TODO fix this parsing so it properly parses inputs in these formats
// - All ISO 8601 dates (2020-01-01T00:00:00Z)
// - Durations from now (20m, 20 minutes, 20 hours, 20 days, 20 weeks)
// - Aliases (now, tomorrow, monday, eod (End of day), see Taskwarrior Synonyms for inspiration: https://taskwarrior.org/docs/dates.html)
fn parse_natural_datetime(input: &str) -> Result<TimeOrDate, &str> {
    // Try parsing as both a fuzzy time and a fuzzy date
    let parsed_time = TimeParser::parse(input);
    let parsed_date = DateParser::parse(input);

    match (parsed_time, parsed_date) {
        (Some(time), Some(_date)) => Ok(TimeOrDate::Time(time)),
        (Some(time), None) => Ok(TimeOrDate::Time(time)),
        (None, Some(date)) => Ok(TimeOrDate::Date(date)),
        (None, None) => Err("Invalid date"),
    }
}

pub fn parse() -> Args {
    Args::parse()
}
