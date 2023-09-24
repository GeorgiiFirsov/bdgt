extern crate dateparser;
extern crate dialoguer;
extern crate itertools;
extern crate libbdgt;
extern crate anyhow;
extern crate chrono;
extern crate clap;

mod console;
mod command;
mod binding;
mod errors;
mod error;
mod misc;


use command::Command;
use error::Result;


/// Macro, that simplifies working with commands.
/// Automatically handles all given commands with their arguments.
macro_rules! handle_commands {
    ( $($command:ty),+ $(,)? ) => {
        match clap::command!()
            .subcommand_required(true)
            .arg_required_else_help(true)
            .propagate_version(true)
            $(.subcommand(<$command>::make_command()))+
            .get_matches()
            .subcommand()
        {
            $(Some((<$command>::VERB, sub_matches)) => <$command>::invoke(sub_matches),)+
            _ => unreachable!("This code is unreachable due to 'subcommand_required' call")
        }
    };
}


/// Runs main app's process.
fn run() -> Result<()> {
    //
    // Just handle available commands in a declarative way 
    // with the nice macro above
    //

    handle_commands!(
        command::Initialize,
        command::Report,
        command::AddTransaction,
        command::AddAccount,
        command::AddCategory,
        command::AddPlan,
        command::RemoveAccount,
        command::RemoveCategory,
        command::RemovePlan,
        command::About,
    )
}


/// Handles and error in app.
/// 
/// For now just prints message into STDERR.
/// 
/// * `error` - error object to handle
fn handle_error<E: std::error::Error>(error: E) {
    eprintln!("{}", error);
}


fn main() {
    match run() {
        Ok(_) => (),
        Err(error) => handle_error(error),
    }
}
