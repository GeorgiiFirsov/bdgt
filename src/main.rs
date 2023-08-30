extern crate libbdgt;
extern crate clap;

mod command;
mod binding;
mod errors;
mod misc;


use command::Command;


/// Runs main app's process.
fn run() -> libbdgt::error::Result<()> {
    //
    // Let's describe available commands in a declarative way
    //

    let matches = clap::command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .propagate_version(true)
        .subcommand(command::Initialize::make_command())
        .subcommand(command::AddTransaction::make_command())
        .get_matches();

    //
    // And now it's time to run
    //

    match matches.subcommand() {
        Some((command::Initialize::VERB, sub_matches)) => command::Initialize::invoke(sub_matches),
        Some((command::AddTransaction::VERB, sub_matches)) => command::AddTransaction::invoke(sub_matches),
        _ => unreachable!("This code is unreachable due to 'subcommand_required' call")
    }
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
