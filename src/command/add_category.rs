use libbdgt::error::Result;
use libbdgt::storage::{Category, CategoryType};

use super::command::{Command, CommandInternal};
use super::common;
use crate::binding;
use crate::misc;


/// Transaction addition command. Adds a new transaction in interactive mode.
pub(crate) struct AddCategory;


impl Command for AddCategory {
    const VERB: &'static str = "add-category";

    const ABOUT: &'static str = "Add a category(s) in interactive mode";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "add several categories one-by-one"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let multi = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        while {
            Self::input_category()
                .and_then(|category| budget.add_category(category))?;

            //
            // If multiple categories requested, then ask if one needs to add another one
            //

            multi && Self::needs_another_category()
        } { /* Intentionally empty */ } 

        Ok(())
    }
}


impl CommandInternal for AddCategory {
    type ParsedArgs = bool;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Self::get_one(matches, "multi")
    }
}


impl AddCategory {
    fn input_category() -> Result<Category> {
        let selection = dialoguer::Select::new()
            .with_prompt("Select what type of category you want")
            .items(&common::category_types())
            .default(0)
            .interact()?;

        let name = misc::input_string_with_prompt("Enter category name: ")?;

        Ok(Category { 
            name: name, 
            category_type: misc::either!(selection == 0 => CategoryType::Income; CategoryType::Outcome)
        })
    }

    fn needs_another_category() -> bool {
        dialoguer::Confirm::new()
            .with_prompt("Do you want to add another category?")
            .default(true)
            .interact()
            .unwrap()
    }
}
