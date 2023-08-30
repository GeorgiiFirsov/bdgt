use libbdgt::error::Result;

use super::command::{Command, CommandInternal};
use crate::binding;


/// Transaction addition command. Adds a new transaction in interactive mode.
pub(crate) struct AddTransaction;


impl Command for AddTransaction {
    const VERB: &'static str = "add";

    const ABOUT: &'static str = "Add a transaction(s) in interactive mode";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "add several transactions one-by-one"))
            .arg(clap::arg!(-f --full "configure all possible transaction(s) options"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let (multi, full) = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        // TODO

        Ok(())
    }
}


impl CommandInternal for AddTransaction {
    type ParsedArgs = (bool /* multi */, bool /* full */);

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        let multi = Self::get_one(matches, "multi")?;
        let full = Self::get_one(matches, "full")?;

        Ok((multi, full))
    }
}