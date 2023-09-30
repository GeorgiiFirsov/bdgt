use libbdgt::storage::Timestamp;

use super::command::{Command, CommandInternal};
use crate::error::{Result, Error};
use crate::timestamp;
use crate::errors;
use crate::misc;


/// Time interval [start, end)
type Interval = (Timestamp, Timestamp);


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Build report for a whole period of time.
    epoch: bool,

    /// Year to build report for.
    year: i32,

    /// Month to build report for.
    month: i32,
}


/// Command for showing application's info.
pub(crate) struct Report;


impl Command for Report {
    const VERB: &'static str = "report";

    const ABOUT: &'static str = "Build a report";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(
                clap::arg!(-e --epoch "build report for he whole period of time")
                    .conflicts_with_all(["month", "year"])
            )
            .arg(
                clap::arg!(-m --month [MONTH] "month to build report for (defaults to current month)")
                    .conflicts_with("epoch")
                    .default_value("0")
                    .value_parser(clap::value_parser!(i32).range(-1..=12))
                    .allow_negative_numbers(true)
                    .long_help(misc::multiline!(
                        "Possible values for MONTH parameter: [-1 .. 12].",
                        " - Positive values denote number of month in year",
                        " - Zero denotes the current month",
                        " - Negative one denotes the previous month",
                    ))
            )
            .arg(
                clap::arg!(-y --year [YEAR] "year to build report for (defaults to current year)")
                    .conflicts_with("epoch")
                    .default_value("0")
                    .value_parser(clap::value_parser!(i32).range(-1..))
                    .allow_negative_numbers(true)
                    .long_help(misc::multiline!(
                        "Possible values for YEAR parameter: -1, 0 or just a year.",
                        " - Zero denotes the current year",
                        " - Negative one denotes the previous year",
                    ))
            )
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let parameters = Self::parse_args(matches)?;
        let interval = Self::time_interval(&parameters)?;
        println!("{:?}", interval);

        Ok(())
    }
}


impl CommandInternal for Report {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        let epoch = Self::get_one(matches, "epoch")?;
        let month = Self::get_one(matches, "month")?;
        let year = Self::get_one(matches, "year")?;

        Ok(Parameters { 
            epoch: epoch, 
            year: year, 
            month: month,
        })
    }
}


impl Report {
    fn time_interval(parameters: &Parameters) -> Result<Option<Interval>> {
        if parameters.epoch {
            return Ok(None);
        }

        //
        // Time interval parameters are parsed according to the 
        // following table:
        //
        // +------+------+--------+------------------------------------------+
        // | Case | Year |  Month | Result                                   |
        // +------+------+--------+------------------------------------------+
        // |    1 |   -1 | 1 - 12 | Report for specific month, last year     |
        // |    2 |   -1 |      0 | Report for last year                     |
        // |    3 |   -1 |     -1 | Not supported                            |
        // |    4 |    0 | 1 - 12 | Report for specific month, current year  |
        // |    5 |    0 |      0 | Report for current month                 |
        // |    6 |    0 |     -1 | Report for previous month                |
        // |    7 |  any | 1 - 12 | Report for specific month, specific year |
        // |    8 |  any |      0 | Report for specific year                 |
        // |    9 |  any |     -1 | Not supported                            |
        // +------+------+--------+------------------------------------------+
        //
        // Report for current year can be obtained by providing the 
        // year explicitly.
        //

        let (month, year) = (parameters.month, parameters.year);
        
        if month == -1 && year != 0 {
            //
            // Cases 3 and 9
            //

            return Err(Error::from_message(errors::INVALID_INTERVAL));
        }

        let duration = match (month, year) {
            (0, y) if y != 0 => {
                //
                // Case 2 and case 8
                //

                timestamp::Duration::Year(1)
            },
            _ => {
                //
                // Rest cases
                //

                timestamp::Duration::Month(1)
            }
        };

        let start = timestamp::make_date(
            timestamp::absolute_year(year), 
            timestamp::absolute_month(month), 
            1)?;

        let end = timestamp::advance_date(&start, duration)?;

        Ok(Some((start, end)))
    }
}
