use libbdgt::storage::{Id, Account};

use super::command::{Command, CommandInternal};
use crate::error::Result;
use crate::console;
use crate::binding;


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Bypass linked account update.
    emergency: bool,

    /// List all transactions for account
    list_all: bool,
}


/// Transaction removal command. Displays multiselect control and then removes selected categories.
pub(crate) struct RemoveTransaction;


impl Command for RemoveTransaction {
    const VERB: &'static str = "remove";

    const ABOUT: &'static str = "Remove selected categories";

    fn aliases(command: clap::Command) -> clap::Command {
        command
            .visible_aliases(["remove-transaction", "remove-tr", "rm", "rm-tr"])
    }

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(--emergency -e "Bypass linked account updates"))
            .arg(clap::arg!(--"list-all" -a "List all transactions for account (transactions for last month are listed by default"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        todo!("implmentation")
    }
}


impl CommandInternal for RemoveTransaction {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(Parameters{
            emergency: Self::get_one(matches, "emergency")?,
            list_all: Self::get_one(matches, "list-all")?
        })
    }
}
