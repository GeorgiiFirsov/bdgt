use libbdgt::error::Result;
use libbdgt::storage::Category;

use super::command::{Command, CommandInternal};
use super::common;
use crate::console;
use crate::binding;


/// Category addition command. Adds a new category in interactive mode.
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

            multi && Self::needs_another_category()?
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
        let selection = console::select_from_with_prompt(&common::category_types(), 
            "Select what type of category you want")?;
            
        let name = console::input_string_with_prompt("Enter category name")?;

        Ok(Category { 
            id: None,
            name: name, 
            category_type: common::category_type_by_index(selection)?
        })
    }

    fn needs_another_category() -> Result<bool> {
        console::confirm_with_prompt("Do you want to add another category?", true)
    }
}
