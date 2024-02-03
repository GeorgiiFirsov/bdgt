use libbdgt::storage::Account;

use super::command::{Command, CommandInternal};
use crate::error::Result;
use crate::console;
use crate::binding;


/// Account addition command. Adds a new account in interactive mode.
pub(crate) struct AddAccount;


impl Command for AddAccount {
    const VERB: &'static str = "add-account";

    const ABOUT: &'static str = "Add an account(s) in interactive mode";

    fn aliases(command: clap::Command) -> clap::Command {
        command
            .visible_alias("add-acc")
    }

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "add several accounts one-by-one"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let multi = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        while {
            budget.add_account(Self::input_account()?)?;

            //
            // If multiple accounts requested, then ask if one needs to add another one
            //

            multi && Self::needs_another_account()?
        } { /* Intentionally empty */ } 

        Ok(())
    }
}


impl CommandInternal for AddAccount {
    type ParsedArgs = bool;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Self::get_one(matches, "multi")
    }
}


impl AddAccount {
    fn input_account() -> Result<Account> {
        let name = console::input_string_with_prompt("Enter account name")?;
        let balance = console::input_number_with_prompt("Enter initial balance")?;

        Ok(Account { 
            id: None,
            name: name, 
            balance: balance
        })
    }

    fn needs_another_account() -> Result<bool> {
        console::confirm_with_prompt("Do you want to add another account?", true)
    }
}
