use libbdgt::error::{Result, Error};
use libbdgt::budget::Budget;
use libbdgt::config::Config;
use libbdgt::crypto::{CryptoEngine, GpgCryptoEngine};
use libbdgt::storage::DbStorage;
use libbdgt::location::{Location, HomeLocation};

use crate::errors;

/// Cryptographic engine type alias for quick engine changes.
type Engine = GpgCryptoEngine;

/// Corresponding key identifier type alias.
type KeyId = <GpgCryptoEngine as CryptoEngine>::KeyId;


/// Public command trait. Each command implements it to be integrated
/// into command line parser in [`crate::run`].
pub(crate) trait Command {
    /// Command name.
    const VERB: &'static str;

    /// Creates a [`clap::Command`] instance for itself.
    /// Can for instance define some arguments for the command.
    fn make_command() -> clap::Command;

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

    fn make_command() -> clap::Command {
        clap::Command::new(Self::VERB)
            .about("Initialize storage and use key_id for sensitive data protection")
            .long_about(concat!(
                "This command MUST be invoked before any other command may be run.\n",
                "Specifier key_id MUST be a valid key identifier for used cryptographic engine.\n",
                "Key MUST be asymmetric and be suitable for encryption and decryption:\n",
                "\t- MUST contain private key;\n",
                "\t- MUST have encryption key usage."
            ))
            .arg(clap::arg!(<key_id> "key identifier for data protection"))
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        //
        // Check for storage existence
        //

        let loc = HomeLocation::new();
        if loc.exists() {
            return Err(Error::from_message_with_extra(
                errors::ALREADY_INITIALIZED, loc.root().to_str().unwrap()));
        }


        //
        // Let's check key presense and validity
        //

        let key_id = Self::parse_args(matches)?;
        let id = KeyId::new(key_id.as_str());

        let mut engine = Engine::new()?;
        engine.lookup_key(&id)?;

        //
        // Key is present and suitable for encryption,
        // now I can create storage
        //

        Config::<Engine>::create(&loc, &id)?;
        DbStorage::create(&loc)?;

        //
        // Just to be nice -- print some information
        //

        println!("Using {} engine of version {}", engine.engine(), engine.version());
        println!("Encryption key identifier: {}", key_id);

        Ok(())
    }
}


impl CommandInternal for Initialize {
    type ParsedArgs = String;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        matches.get_one::<String>("key_id")
            .map(|r| r.to_owned())
            .ok_or(Error::from_message_with_extra(errors::PARSE_ERROR, Self::VERB))
    }
}
