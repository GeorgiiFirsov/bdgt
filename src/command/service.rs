use super::command::{Command, CommandInternal};
use crate::error::Result;
use crate::console;
use crate::binding;


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Skip confirmations.
    force: bool,

    /// Cleanup removed items.
    cleanup: bool,
}


/// Service mode command.
pub(crate) struct Service;


impl Command for Service {
    const VERB: &'static str = "service";

    const ABOUT: &'static str = "Enter service mode (for advanced users)";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(-f --force "skip all confirmations"))
            .arg(clap::arg!(--cleanup "clear all removed transactions"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let parameters = Self::parse_args(matches)?;

        //
        // Kindly ask if user actually needs to enter service mode
        //

        if !Self::confirm_with_prompt("Do you actually NEED to enter service mode?", parameters.force)? {
            return Ok(());
        }

        //
        // Well, ok... Perform requested actions
        //

        let budget = binding::open_budget()?;

        if parameters.cleanup {
            Self::cleanup(&budget, parameters.force)?;
        }

        Ok(())
    }
}


impl CommandInternal for Service {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(Parameters{
            force: Self::get_one(matches, "force")?,
            cleanup: Self::get_one(matches, "cleanup")?
        })
    }
}


impl Service {
    fn cleanup(budget: &binding::Budget, force: bool) -> Result<()> {
        if !Self::confirm_with_prompt("You will be unable to sync removed items. Proceed?", force)? {
            return Ok(());
        }
        
        budget.clean_removed()?;
        println!("All removed items are completely deleted.");

        Ok(())
    }

    fn confirm_with_prompt(prompt: &str, force: bool) -> Result<bool> {
        if force {
            return Ok(true);
        }

        console::confirm_with_prompt(prompt, false)
    }
}
