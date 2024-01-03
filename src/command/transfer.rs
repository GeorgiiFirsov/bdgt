use libbdgt::storage::{Account, Id};

use super::command::{Command, CommandInternal};
use crate::error::{Result, Error};
use crate::console;
use crate::binding;
use crate::errors;


/// Transfer addition command. Adds a new transfer transactions in interactive mode.
pub(crate) struct Transfer;


impl Command for Transfer {
    const VERB: &'static str = "transfer";

    const ABOUT: &'static str = "Add transfer transactions in interactive mode";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "transfer between several accounts in a row"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let multi = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        let accounts = budget.accounts()?;

        if accounts.is_empty() {
            return Err(Error::from_message(errors::NO_ACCOUNTS));
        }

        while {
            let (amount, from, to) = Self::input_transfer(&accounts)?;

            if from != to {
                budget.add_transfer(amount, from, to)?;
            }
            else {
                println!("FROM and TO accounts are the same, skipped...");
            }

            //
            // If multiple transfers requested, then ask if one needs to add another one
            //

            multi && Self::needs_another_transfer()?
        } { /* Intentionally empty */ } 

        Ok(())
    }
}


impl CommandInternal for Transfer {
    type ParsedArgs = bool;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Self::get_one(matches, "multi")
    }
}


impl Transfer {
    fn input_transfer(accounts: &Vec<Account>) -> Result<(isize, Id, Id)> {
        //
        // Ask for 'from' and 'to' accounts
        //

        let printable_accounts: Vec<_> = accounts
            .iter()
            .map(|account| {
                format!("{} (balance: {})", account.name, console::colorize_amount(account.balance))
            })
            .collect();

        let selection = console::select_from_with_prompt(&printable_accounts, 
            "Which account is intended to transfer FROM?")?;

        let from = accounts[selection].id.unwrap();

        let selection = console::select_from_with_prompt(&printable_accounts, 
            "Which account is intended to transfer TO?")?;

        let to = accounts[selection].id.unwrap();

        //
        // Ask for amount. It will be normalized later in `libbdgt::core::Budget`.
        //

        let amount = console::input_number_with_prompt("Amount")?;

        Ok((amount, from, to))
    }

    fn needs_another_transfer() -> Result<bool> {
        console::confirm_with_prompt("Do you want to add another transfer?", true)
    }
}
