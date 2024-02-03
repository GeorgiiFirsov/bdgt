use super::command::{Command, CommandInternal};
use super::common;
use crate::error::Result;
use crate::console;
use crate::binding;


/// Category removal command. Displays multiselect control and then removes selected categories.
pub(crate) struct RemoveCategory;


impl Command for RemoveCategory {
    const VERB: &'static str = "remove-category";

    const ABOUT: &'static str = "Remove selected categories";

    fn aliases(command: clap::Command) -> clap::Command {
        command
            .visible_aliases(["rm-category", "rm-cat"])
    }

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        let budget = binding::open_budget()?;
        let categories = budget.categories()?;

        if categories.is_empty() {
            //
            // Returning here, nothing to do for now
            //

            return Ok(());
        }

        let printable_categories: Vec<_> = categories
            .iter()
            .map(|category| {
                format!("{} ({})", category.name, common::category_type_to_string(category.category_type))
            })
            .collect();

        let selection = console::select_multiple_from_with_prompt(
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
