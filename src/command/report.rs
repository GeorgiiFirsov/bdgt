use libbdgt::error::Result;

use super::command::{Command, CommandInternal};


/// Command for showing application's info.
pub(crate) struct Report;


impl Command for Report {
    const VERB: &'static str = "report";

    const ABOUT: &'static str = "Build a report";

    fn add_args(command: clap::Command) -> clap::Command {
        command
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
