use super::command::{Command, CommandInternal};
use crate::error::Result;
use crate::binding;
use crate::misc;


/// Command for showing application's info.
pub(crate) struct About;


impl Command for About {
    const VERB: &'static str = "about";

    const ABOUT: &'static str = "Display information about the application";

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        //
        // Collect info about application. This information is always available.
        //

        let app = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");  
        let authors = env!("CARGO_PKG_AUTHORS");
        let homepage = env!("CARGO_PKG_HOMEPAGE");
        let license = env!("CARGO_PKG_LICENSE");
        let engine_info = binding::query_engine_info()?;

        //
        // And just print it in a pretty way!
        //

        println!(misc::multiline!(
                "{} {}",                            // App name and version
                "Authors: {}",                      // List of application's authors
                "Home page: {}",                    // Home page of the application
                "Distributed under {}",             // License information
                "",
                "Cryptographic engine: {} ({})",    // Cryptographic engine information
            ), 
            app, version, 
            authors,
            homepage,
            license,
            engine_info.0, engine_info.1
        );

        //
        // Now if local instance is initialized, I need to display some
        // information about it too
        //

        if let Ok(budget) = binding::open_budget() {
            println!(misc::multiline!(
                    "Local instance identifier: {}",
                    "Encryption key identifier: {}"
                ),
                budget.instance_id(), 
                budget.key_id()
            );
        }

        Ok(())
    }
}


impl CommandInternal for About {
    type ParsedArgs = ();

    fn parse_args(_matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(())
    }
}
