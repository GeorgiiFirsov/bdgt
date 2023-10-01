use super::command::{Command, CommandInternal};
use crate::error::Result;
use crate::binding;
use crate::misc;


/// Initialization command. Creates a new storage.
pub(crate) struct Initialize;


impl Command for Initialize {
    const VERB: &'static str = "init";

    const ABOUT: &'static str = "Initialize storage and use KEY_ID for sensitive data protection";

    const LONG_ABOUT: &'static str = misc::multiline!(
        "This command MUST be invoked before any other command may be run.",
        "Specifier KEY_ID MUST be a valid key identifier for used cryptographic engine.",
        "Key MUST be asymmetric and be suitable for encryption and decryption:",
        "\t- MUST contain private key;",
        "\t- MUST have encryption key usage."
    );

    fn add_args(command: clap::Command) -> clap::Command {
        command.arg(clap::arg!(<KEY_ID> "key identifier for data protection"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        //
        // Parse args and run initialization
        //

        let key_id = Self::parse_args(matches)?;
        let budget = binding::initialize_budget(&key_id)?;

        //
        // Just to be nice -- print some information
        //

        println!(misc::multiline!(
                "Cryptographic engine: {} ({})",
                "Local instance identifier: {}",
                "Encryption key identifier: {}",
            ),
            budget.engine(), budget.engine_version(), 
            budget.instance_id(),
            budget.key_id()
        );

        Ok(())
    }
}


impl CommandInternal for Initialize {
    type ParsedArgs = String;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Self::get_one(matches, "KEY_ID")
    }
}
