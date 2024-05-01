use libbdgt::datetime::{Clock, Timestamp};
use libbdgt::storage::{Account, Id};

use super::command::{Command, CommandInternal};
use crate::error::{Result, Error};
use crate::console;
use crate::binding;
use crate::errors;


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Perform multiple transfers in a row.
    multi: bool,

    /// Input all fields.
    full: bool,
}


/// Transfer addition command. Adds a new transfer transactions in interactive mode.
pub(crate) struct Transfer;


impl Command for Transfer {
    const VERB: &'static str = "transfer";

    const ABOUT: &'static str = "Add transfer transactions in interactive mode";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "transfer between several accounts in a row"))
            .arg(clap::arg!(-f --full "configure all possible transaction(s) options"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let parameters = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        let accounts = budget.accounts()?;

        if accounts.is_empty() {
            return Err(Error::from_message(errors::NO_ACCOUNTS));
        }

        while {
            let (amount, from, to, timestamp) = Self::input_transfer(parameters.full, &accounts)?;

            if from != to {
                budget.add_transfer(amount, from, to, timestamp)?;
            }
            else {
                println!("FROM and TO accounts are the same, skipped...");
            }

            //
            // If multiple transfers requested, then ask if one needs to add another one
            //

            parameters.multi && Self::needs_another_transfer()?
        } { /* Intentionally empty */ } 

        Ok(())
    }
}


impl CommandInternal for Transfer {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        let multi = Self::get_one(matches, "multi")?;
        let full = Self::get_one(matches, "full")?;

        Ok(Parameters {
            multi: multi,
            full: full
        })
    }
}


impl Transfer {
    fn input_transfer(full: bool, accounts: &Vec<Account>) -> Result<(isize, Id, Id, Timestamp)> {
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

        let timestamp = if full {
            let datetime = console::input_string_with_prompt("Enter date and time of the transfer")?;
            dateparser::parse(&datetime)?
        }
        else {
            Clock::now()
        };

        Ok((amount, from, to, timestamp))
    }

    fn needs_another_transfer() -> Result<bool> {
        console::confirm_with_prompt("Do you want to add another transfer?", true)
    }
}
