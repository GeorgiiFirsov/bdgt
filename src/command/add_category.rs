use libbdgt::datetime::Clock;
use libbdgt::core::InstanceId;
use libbdgt::storage::{Category, MetaInfo};

use super::command::{Command, CommandInternal};
use super::common;
use crate::error::Result;
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

        let instance_id = budget.instance_id();

        while {
            budget.add_category(&Self::input_category(instance_id)?)?;

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
    fn input_category(instance_id: &InstanceId) -> Result<Category> {
        let selection = console::select_from_with_prompt(&common::category_types(), 
            "Select what type of category you want")?;
            
        let name = console::input_string_with_prompt("Enter category name")?;

        Ok(Category { 
            id: None,
            name: name, 
            category_type: common::category_type_by_index(selection)?,
            meta_info: MetaInfo::new(instance_id, Some(Clock::now()), None, None)
        })
    }

    fn needs_another_category() -> Result<bool> {
        console::confirm_with_prompt("Do you want to add another category?", true)
    }
}
