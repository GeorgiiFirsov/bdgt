extern crate libbdgt;
extern crate clap;

mod commands;
mod binding;
mod errors;


use commands::Command;


/// Runs main app's process.
fn run() -> libbdgt::error::Result<()> {
    //
    // Let's describe available commands in a declarative way
    //

    let matches = clap::command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .propagate_version(true)
        .subcommand(commands::Initialize::make_command())
        .get_matches();

    //
    // And now it's time to run
    //

    match matches.subcommand() {
        Some((commands::Initialize::VERB, sub_matches)) => commands::Initialize::invoke(sub_matches),
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
