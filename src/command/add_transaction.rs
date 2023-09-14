use libbdgt::error::{Result, Error};
use libbdgt::storage::{Transaction, CategoryType, Category, Account};

use super::command::{Command, CommandInternal};
use crate::binding;


/// Structure with command parameters
pub(crate) struct Parameters {
    /// Input multiple transactions in a row 
    multi: bool,

    /// Input all trasaction fields
    full: bool,

    /// Input transaction of specific category
    category_type: Option<CategoryType>,
}


/// Transaction addition command. Adds a new transaction in interactive mode.
pub(crate) struct AddTransaction;


impl Command for AddTransaction {
    const VERB: &'static str = "add";

    const ABOUT: &'static str = "Add a transaction(s) in interactive mode";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "add several transactions one-by-one"))
            .arg(clap::arg!(-f --full "configure all possible transaction(s) options"))
            .arg(
                clap::arg!(-i --income "add income transaction(s)")
                    .conflicts_with("outcome")
            )
            .arg(
                clap::arg!(-o --outcome "add outcome transaction(s)")
                    .conflicts_with("income")
            )
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let parameters = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        let accounts = budget.accounts()?;
        let categories = match parameters.category_type {
            Some(category_type) => budget.categories_of(category_type)?,
            None => budget.categories()?
        };

        while {
            Self::input_transaction(parameters.full, &accounts, &categories, parameters.category_type)
                .and_then(|transaction| budget.add_transaction(transaction))?;

            //
            // If multiple transactions requested, then ask if one needs to add another one
            //

            parameters.multi && Self::needs_another_transaction()
        } { /* Intentionally empty */ } 

        Ok(())
    }
}


impl CommandInternal for AddTransaction {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        let multi = Self::get_one(matches, "multi")?;
        let full = Self::get_one(matches, "full")?;

        let income = Self::get_one(matches, "income")?;
        let outcome = Self::get_one(matches, "outcome")?;

        let category_type = match (income, outcome) {
            (true, false) => Some(CategoryType::Income),
            (false, true) => Some(CategoryType::Outcome),
            _ => None
        };

        Ok(Parameters { 
            multi: multi, 
            full: full, 
            category_type: category_type 
        })
    }
}


impl AddTransaction {
    fn input_transaction(full: bool, accounts: &Vec<Account>, categories: &Vec<Category>, category_type: Option<CategoryType>) -> Result<Transaction> {
        // TODO
        Err(Error::from_message("not implemented"))
    }

    fn needs_another_transaction() -> bool {
        // TODO
        false
    }
}
