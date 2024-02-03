use secrecy::ExposeSecret;

use super::command::{Command, CommandInternal};
use crate::error::{Error, Result};
use crate::binding;


/// Command for showing application's info.
pub(crate) struct Sync;


impl Command for Sync {
    const VERB: &'static str = "sync";

    const ABOUT: &'static str = "Synchronize with remote";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(--"set-remote" <REMOTE> "Set (with replacement) a new remote URL and sync"))
            .arg(
                clap::arg!(--"no-sync" "Do not perform syn after setting a new remote URL")
                    .requires("set-remote")
            )
    }

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        let budget = binding::open_budget()?;
        
        let mut input = pinentry::PassphraseInput::with_default_binary()
            .ok_or(Error::from_message("No pinentry binary found"))?;
        
        let passphrase = input
            .with_description("Please enter the passphrase to unlock sync storage")
            .with_prompt("Passphrase:")
            .required("Passphrase is required")
            .interact()?;
        
        budget.perform_sync(passphrase.expose_secret().as_bytes())?;

        Ok(())
    }
}


impl CommandInternal for Sync {
    type ParsedArgs = ();

    fn parse_args(_matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        // TODO
        Ok(())
    }
}
