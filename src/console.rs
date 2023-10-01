use std::fmt::Write;

use crate::error::Result;


/// Trait that provides a method of writing something into a [`minus::Pager`].
pub(crate) trait WritePaged {
    ///
    fn write_paged(&self, pager: &mut minus::Pager) -> Result<()>;
}


/// Implementation of [`WritePaged`] for [`prettytable::Table`].
/// If the table is styled using [`colored`] create functions, 
/// the function preserves all styles.
impl WritePaged for prettytable::Table {
    fn write_paged(&self, pager: &mut minus::Pager) -> Result<()> {
        let mut buffer = Vec::new();
        self.print(&mut buffer)?;

        let string_buffer = String::from_utf8(buffer)?;
        pager.write_str(&string_buffer)?;

        Ok(())
    }
}


/// Implementation of [`WritePaged`] for [`String`]. The function
/// preserves styles set with help of [`colored`] crate.
impl WritePaged for String {
    fn write_paged(&self, pager: &mut minus::Pager) -> Result<()> {
        pager.write_str(&self)?;

        Ok(())
    }
}


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
