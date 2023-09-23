use libbdgt::error::Result;

use super::command::{Command, CommandInternal};
use crate::console;
use crate::binding;


/// Plan removal command. Displays multiselect control and then removes selected plans.
pub(crate) struct RemovePlan;


impl Command for RemovePlan {
    const VERB: &'static str = "remove-plan";

    const ABOUT: &'static str = "Remove selected plans";

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        let budget = binding::open_budget()?;
        let plans = budget.plans()?;

        if plans.is_empty() {
            //
            // Returning here, nothing to do for now
            //

            return Ok(());
        }

        let printable_plans: Vec<_> = plans
            .iter()
            .map(|plan| &plan.name)
            .collect();

        let selection = console::select_multiple_from_with_prompt(
            &printable_plans, "Select plans to remove")?;

        for idx in selection {
            let plan = &plans[idx];
            match budget.remove_plan(plan.id.unwrap()) {
                Ok(_) => {},
                Err(e) => eprintln!("Cannot remove plan '{}': {}", plan.name, e)
            }
        }

        Ok(())
    }
}


impl CommandInternal for RemovePlan {
    type ParsedArgs = ();

    fn parse_args(_matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(())
    }
}
