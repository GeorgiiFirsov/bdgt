use libbdgt::error::{Result, Error};
use libbdgt::storage::{Plan, Category, CategoryType};

use super::command::{Command, CommandInternal};
use super::common;
use crate::binding;
use crate::errors;
use crate::misc;


/// Plan addition command. Adds a new plan(s) in interactive mode.
pub(crate) struct AddPlan;


impl Command for AddPlan {
    const VERB: &'static str = "add-plan";

    const ABOUT: &'static str = "Add a plan(s) in interactive mode";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-m --multi "add several plans one-by-one"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let multi = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;

        //
        // Plans are supposed to be only for spendings
        //

        let categories = budget.categories_of(CategoryType::Outcome)?;

        if categories.is_empty() {
            return Err(Error::from_message(errors::NO_CATEGORIES));
        }

        while {
            Self::input_plan(&categories)
                .and_then(|plan| budget.add_plan(plan))?;

            //
            // If multiple plans requested, then ask if one needs to add another one
            //

            multi && Self::needs_another_plan()?
        } { /* Intentionally empty */ } 

        Ok(())
    }
}


impl CommandInternal for AddPlan {
    type ParsedArgs = bool;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Self::get_one(matches, "multi")
    }
}


impl AddPlan {
    fn input_plan(categories: &Vec<Category>) -> Result<Plan> {
        //
        // Ask for category
        //

        let printable_categories: Vec<_> = categories
            .iter()
            .map(|c| {
                format!("{} ({})", c.name, common::category_type_to_string(c.category_type))
            })
            .collect();

        let selection = misc::select_from_with_prompt(&printable_categories, 
            "Which category does plan belong to?")?;

        let category = &categories[selection];

        //
        // Ask for plan name and amount limit. Amount limit is intended to be positive.
        // If it doesn't, then I just negate the value.
        //

        let name = misc::input_string_with_prompt("Plan name")?;
        let amount_limit = misc::input_number_with_prompt("Amount limit (a negative value will be negated)")
            .map(isize::abs)?;

        Ok(Plan {
            id: None,
            category_id: category.id.unwrap(),
            name: name,
            amount_limit: amount_limit
        })
    }

    fn needs_another_plan() -> Result<bool> {
        misc::confirm_with_prompt("Do you want to add another plan?", true)
    }
}