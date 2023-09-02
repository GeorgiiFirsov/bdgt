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
    let input = dialoguer::Input::new()
        .with_prompt(prompt)
        .allow_empty(false)
        .interact_text()?;

    Ok(input)
}


/// Displays selection menu using given items and prompt.
/// 
/// * `items` - items to select from
/// * `prompt` - string to display before input
pub(crate) fn select_from_with_prompt<T, S>(items: &[T], prompt: S) -> Result<usize>
where
    T: ToString,
    S: Into<String>
{
    let selection = dialoguer::Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()?;

    Ok(selection)
}


/// Displays confirmation menu with given default selection and prompt.
/// 
/// * `prompt` - string to display before input
/// * `default` - default selection
pub(crate) fn confirm_with_prompt<S>(prompt: S, default: bool) -> Result<bool>
where
    S: Into<String>
{
    let confirmation = dialoguer::Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()?;

    Ok(confirmation)
}
