use libbdgt::error::{Result, Error};
use libbdgt::storage::{Id, Transaction};

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

        while {
            Self::input_transaction(full)
                .and_then(|(account, transaction)| budget.add_transaction(account, transaction))?;

            //
            // If multiple transactions requested, then ask if one needs to add another one
            //

            multi && Self::needs_another_transaction()
        } { /* Intentionally empty */ } 

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


impl AddTransaction {
    fn input_transaction(full: bool) -> Result<(Id, Transaction)> {
        // TODO
        Err(Error::from_message("not implemented"))
    }

    fn needs_another_transaction() -> bool {
        // TODO
        false
    }
}
