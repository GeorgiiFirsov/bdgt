use libbdgt::error::Result;

use super::command::{Command, CommandInternal};
use crate::binding;
use crate::misc;


/// Command for showing application's info.
pub(crate) struct Report;


impl Command for Report {
    const VERB: &'static str = "report";

    const ABOUT: &'static str = "Build a report";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(
                clap::arg!(--accounts "summary information for all accounts")
                    .conflicts_with_all(["account"])
            )
            .arg(
                clap::arg!(-a --account [account_id] "summary information for a chosen account")
                    .conflicts_with_all(["accounts"])
            )
            .arg(
                clap::arg!(--categories "summary information for all ca")
            )
    }

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        Ok(())
    }
}


impl CommandInternal for Report {
    type ParsedArgs = ();

    fn parse_args(_matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(())
    }
}
