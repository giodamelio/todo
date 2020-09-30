use clap::{Clap, crate_authors, crate_version, crate_description, AppSettings};
use chrono::{NaiveDate, NaiveTime};
use date_time_parser::{DateParser, TimeParser};

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!("\n"), about = crate_description!())]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(about = "Add a TODO", setting = AppSettings::TrailingVarArg)]
    Add(Add)
}

#[derive(Clap, Debug)]
struct Add {
    #[clap(short, long, parse(try_from_str = parse_natural_datetime), about = "The datetime that the TODO is due")]
    due: Option<TimeOrDate>,

    #[clap(multiple = true)]
    message: Vec<String>,
}

#[derive(Debug)]
enum TimeOrDate {
    Time (NaiveTime),
    Date (NaiveDate),
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

fn main() {
    let args = Args::parse();

    println!("Args: {:?}", args);
}