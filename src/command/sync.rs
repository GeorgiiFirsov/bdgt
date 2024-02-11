use secrecy::ExposeSecret;

use super::command::{Command, CommandInternal};
use crate::error::{Error, Result};
use crate::binding;


/// Structure with command parameters.
pub(crate) struct Parameters  {
    /// New remote URL (if any).
    remote: Option<String>,

    /// Don't sync after URL change.
    no_sync: bool,
}


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

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let parameters = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;
        
        if let Some(remote) = parameters.remote {
            budget.set_remote_url(&remote)?
        }

        if !parameters.no_sync {
            Self::perform_sync(&budget)?;
        }

        Ok(())
    }
}


impl CommandInternal for Sync {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(Parameters{
            remote: Self::get_one_opt(matches, "set-remote"),
            no_sync: Self::get_one(matches, "no-sync")
                .unwrap_or(false),
        })
    }
}


impl Sync {
    fn perform_sync(budget: &binding::Budget) -> Result<()> {
        let mut input = pinentry::PassphraseInput::with_default_binary()
            .ok_or(Error::from_message("No pinentry binary found"))?;
        
        let passphrase = input
            .with_description("Please enter the passphrase to unlock sync storage")
            .with_prompt("Passphrase:")
            .required("Passphrase is required")
            .interact()?;
        
        println!("Symmetric key generation may take a while...");
        budget.perform_sync(passphrase.expose_secret().as_bytes())?;

        Ok(())
    }
}
