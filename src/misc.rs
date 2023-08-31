use std::io::Write;

use libbdgt::error::Result;


/// Ternary operator emulation.
/// 
/// # Examples
/// 
/// ```rust
/// let val = either!(is_beautiful()
///     => beautiful_process()
///      ; awesome_process()
/// )
/// ```
macro_rules! either {
    ( $condition:expr => $true:expr ; $false:expr ) => {
        if $condition {
            $true
        }
        else {
            $false
        }
    }
}

pub(crate) use either;


/// Reads a string from STDIN with printing a prompt before.
/// 
/// * `prompt` - string to display before input
pub(crate) fn input_string_with_prompt<S>(prompt: S) -> Result<String>
where
    S: Into<String>
{
    print!("{}", prompt.into());
    std::io::stdout()
        .flush()?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)?;

    Ok(input)
}
