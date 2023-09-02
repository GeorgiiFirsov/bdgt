use itertools::Itertools;

use libbdgt::error::Result;

use super::command::{Command, CommandInternal};
use crate::binding;
use super::common;
use crate::misc;


/// Category removal command. Displays multiselect control and then removes selected categories.
pub(crate) struct RemoveCategory;


impl Command for RemoveCategory {
    const VERB: &'static str = "remove-category";

    const ABOUT: &'static str = "Remove selected categories";

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        let budget = binding::open_budget()?;
        let categories = budget.categories()?;

        let printable_categories: Vec<_> = categories
            .iter()
            .sorted_by_key(|c| c.category_type)
            .map(|c| {
                format!("{} ({})", c.name, common::category_type_to_string(c.category_type))
            })
            .collect();

        let selection = misc::select_multiple_from_with_prompt(
            &printable_categories, "Select categories to remove")?;

        for idx in selection {
            let category = &categories[idx];
            match budget.remove_category(category.id.unwrap()) {
                Ok(_) => {},
                Err(e) => eprintln!("Cannot remove category '{}': {}", category.name, e)
            }
        }

        Ok(())
    }
}


impl CommandInternal for RemoveCategory {
    type ParsedArgs = ();

    fn parse_args(_matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(())
    }
}
