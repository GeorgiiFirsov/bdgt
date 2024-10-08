use libbdgt::datetime::Clock;
use libbdgt::storage::{Transaction, CategoryType, Category, Account, MetaInfo};

use super::command::{Command, CommandInternal};
use super::common;
use crate::error::{Result, Error};
use crate::console;
use crate::binding;
use crate::errors;


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Input multiple transactions in a row.
    multi: bool,

    /// Input all trasaction fields.
    full: bool,

    /// Input transaction of specific category.
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

        if accounts.is_empty() {
            return Err(Error::from_message(errors::NO_ACCOUNTS));
        }

        let categories = match parameters.category_type {
            Some(category_type) => budget.categories_of(category_type)?,
            None => budget.categories()?
        };

        if categories.is_empty() {
            return Err(Error::from_message(errors::NO_CATEGORIES));
        }

        while {
            budget.add_transaction(&Self::input_transaction(parameters.full, &accounts, &categories)?)?;

            //
            // If multiple transactions requested, then ask if one needs to add another one
            //

            parameters.multi && Self::needs_another_transaction()?
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
    fn input_transaction(full: bool, accounts: &Vec<Account>, categories: &Vec<Category>) -> Result<Transaction> {
        //
        // Ask for category
        //

        let printable_categories: Vec<_> = categories
            .iter()
            .map(|category| {
                format!("{} ({})", category.name, common::category_type_to_string(category.category_type))
            })
            .collect();

        let selection = console::select_from_with_prompt(&printable_categories, 
            "Which category does transaction belong to?")?;

        let category = &categories[selection];

        //
        // Ask for account
        //

        let printable_accounts: Vec<_> = accounts
            .iter()
            .map(|account| {
                format!("{}", account.name)
            })
            .collect();

        let selection = console::select_from_with_prompt(&printable_accounts, 
            "Which account does transaction belong to?")?;

        let account = &accounts[selection];

        //
        // Ask for description, amount and timestamp if necessary and that's it
        // Amount will be normalized according to selected category
        //

        let description = console::input_string_with_prompt("Description")?;
        let amount = console::input_number_with_prompt("Amount (sign will be selected based on category)")?;
        let amount = common::normalize_amount_by_category(amount, category.category_type);

        let timestamp = if full {
            let datetime = console::input_string_with_prompt("Enter date and time of the transaction")?;
            dateparser::parse(&datetime)?
        }
        else {
            Clock::now()
        };

        Ok(Transaction {
            id: None,
            timestamp: timestamp,
            description: description,
            account_id: account.id.unwrap(),
            category_id: category.id.unwrap(),
            amount: amount,
            meta_info: MetaInfo::new(Some(Clock::now()), None, None)
        })
    }

    fn needs_another_transaction() -> Result<bool> {
        console::confirm_with_prompt("Do you want to add another transaction?", true)
    }
}
