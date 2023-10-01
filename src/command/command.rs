use crate::error::{Error, Result};
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
pub(crate) trait CommandInternal {
    /// Type of parsed arguments.
    type ParsedArgs;

    /// Arguments parser.
    /// 
    /// * `matches` - set of provided arguments to parse
    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs>;

    /// Parses single value for a given argument.
    /// 
    /// * `matches` - set of provided arguments to parse
    /// * `name` - name of an argument to parse value for
    fn get_one<T>(matches: &clap::ArgMatches, name: &str) -> Result<T>
    where
        T: ToOwned<Owned = T> + Clone + Send + Sync + 'static
    {
        Self::get_one_opt::<T>(matches, name)
            .ok_or(Error::from_message_with_extra(errors::PARSE_ERROR, name))
    }

    /// Parses single optional value for a given argument.
    /// 
    /// * `matches` - set of provided arguments to parse
    /// * `name` - name of an argument to parse value for
    fn get_one_opt<T>(matches: &clap::ArgMatches, name: &str) -> Option<T>
    where
        T: ToOwned<Owned = T> + Clone + Send + Sync + 'static
    {
        matches.get_one::<T>(name)
            .map(T::to_owned)
    }
}
