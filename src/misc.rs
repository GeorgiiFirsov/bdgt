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


/// Multiline string builder from arbitrary literals.
/// 
/// Inserts line feed between literals passed to the macro.
/// No line feed is inserted after a literal, if it is the last
/// one in the list. 
/// 
/// Returns an empty string, when no arguments are provided.
/// 
/// # Examples
/// 
/// ```rust
/// let s1 = multiline!("test 1");
/// let s2 = multiline!("test 1", "test 2");
/// 
/// // Will print:
/// // test 1
/// println!("{}", s1);
/// 
/// // Will print:
/// // test 1
/// // test 2
/// println!("{}", s2);
/// ```
macro_rules! multiline {
    () => {
        ""
    };
    ( $fe:expr, $($e:expr),+ $(,)? ) => {
        concat!(
            $fe, $("\n", $e,)+
        )
    };
}

pub(crate) use multiline;


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


/// Reads an `isize` from STDIN with printing a prompt before.
/// 
/// Defaults to 0.
/// 
/// * `prompt` - string to display before input
pub(crate) fn input_number_with_prompt<S>(prompt: S) -> Result<isize>
where
    S: Into<String>
{
    let input = dialoguer::Input::new()
        .with_prompt(prompt)
        .with_initial_text("0")
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


/// Displays selection menu with multiple selection using given items and prompt.
/// 
/// * `items` - items to select from
/// * `prompt` - string to display before input
pub(crate) fn select_multiple_from_with_prompt<T, S>(items: &[T], prompt: S) -> Result<Vec<usize>>
where
    T: ToString,
    S: Into<String>
{
    let selection = dialoguer::MultiSelect::new()
        .with_prompt(prompt)
        .items(items)
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
