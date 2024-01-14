use super::command::{Command, CommandInternal};
use crate::error::Result;


/// Command for showing application's info.
pub(crate) struct Sync;


impl Command for Sync {
    const VERB: &'static str = "sync";

    const ABOUT: &'static str = "Synchronize with remote";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(clap::arg!(--"set-remote" <REMOTE> "Set (with replacement) remote URL"))
    }

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        // TODO
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
