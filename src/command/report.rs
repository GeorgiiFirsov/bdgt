use super::command::{Command, CommandInternal};
use crate::error::Result;
use crate::misc;


/// Type of report.
pub(crate) enum ReportType {
    /// Build report for account(s).
    Accounts,

    /// Build report for category(s).
    Categories,

    /// Build report for plan(s).
    Plans,
}


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Build report for a whole period of time.
    epoch: bool,

    /// Year to build report for.
    year: i16,

    /// Month to build report for.
    month: i8,

    /// Type of report to build.
    report_type: ReportType,
}


/// Command for showing application's info.
pub(crate) struct Report;


impl Command for Report {
    const VERB: &'static str = "report";

    const ABOUT: &'static str = "Build a report";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-e --epoch "build report for he whole period of time"))
            .arg(
                clap::arg!(-m --month [MONTH] "month to build report for (defaults to current month)")
                    .conflicts_with("epoch")
                    .default_value("0")
                    .value_parser(clap::value_parser!(i8))
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
                    .value_parser(clap::value_parser!(i16))
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
        println!("{}, {}, {}", parameters.epoch, parameters.year, parameters.month);

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
            report_type: ReportType::Accounts // TODO
        })
    }
}
