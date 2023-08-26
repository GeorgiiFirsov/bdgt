use libbdgt::error::{Error, Result};

use crate::binding;
use crate::errors;


/// Public command trait. Each command implements it to be integrated
/// into command line parser in [`crate::run`].
pub(crate) trait Command {
    /// Command name.
    const VERB: &'static str;

    /// Short description.
    const ABOUT: &'static str;

    /// Long description.
    /// 
    /// By default it is empty.
    const LONG_ABOUT: &'static str = "";

    /// Creates a [`clap::Command`] instance for itself.
    /// 
    /// Can for instance define some arguments for the command.
    /// By default provides short and long description and calls
    /// arguments setup.
    fn make_command() -> clap::Command {
        Self::add_args(
            clap::Command::new(Self::VERB)
                .about(Self::ABOUT)
                .long_about(Self::LONG_ABOUT)   
        )
    }

    /// Adds necessary arguments to a command.
    /// 
    /// By default does nothing.
    fn add_args(command: clap::Command) -> clap::Command {
        command
    }

    /// Invocation of the command.
    /// 
    /// * `matches` - set of provided arguments
    fn invoke(matches: &clap::ArgMatches) -> Result<()>;
}


/// Internal command trait. Provides proper argument parsing, that
/// can be used in [`Command::invoke`].
trait CommandInternal {
    /// Type of parsed arguments.
    type ParsedArgs;

    /// Arguments parser.
    /// 
    /// * `matches` - set of provided arguments to parse
    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs>;
}


/// Initialization command. Creates a new storage.
pub(crate) struct Initialize;


impl Command for Initialize {
    const VERB: &'static str = "init";

    const ABOUT: &'static str = "Initialize storage and use key_id for sensitive data protection";

    const LONG_ABOUT: &'static str = concat!(
        "This command MUST be invoked before any other command may be run.\n",
        "Specifier key_id MUST be a valid key identifier for used cryptographic engine.\n",
        "Key MUST be asymmetric and be suitable for encryption and decryption:\n",
        "\t- MUST contain private key;\n",
        "\t- MUST have encryption key usage."
    );

    fn add_args(command: clap::Command) -> clap::Command {
        command.arg(clap::arg!(<key_id> "key identifier for data protection"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        //
        // Parse args and run initialization
        //

        let key_id = Self::parse_args(matches)?;
        let budget = binding::initialize(&key_id)?;

        //
        // Just to be nice -- print some information
        //

        println!("Using {} engine of version {}", budget.engine(), budget.engine_version());
        println!("Encryption key identifier: {}", key_id);

        Ok(())
    }
}


impl CommandInternal for Initialize {
    type ParsedArgs = String;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        matches.get_one::<String>("key_id")
            .map(String::to_owned)
            .ok_or(Error::from_message_with_extra(errors::PARSE_ERROR, Self::VERB))
    }
}
