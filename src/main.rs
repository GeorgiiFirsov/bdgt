extern crate libbdgt;


/// Runs main app's process.
fn run() -> libbdgt::error::Result<()> {
    //
    // TODO
    //

    Ok(())
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
