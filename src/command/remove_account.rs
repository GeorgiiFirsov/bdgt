use libbdgt::error::Result;

use super::command::{Command, CommandInternal};
use crate::console;
use crate::binding;


/// Account removal command. Displays multiselect control and then removes selected accounts.
pub(crate) struct RemoveAccount;


impl Command for RemoveAccount {
    const VERB: &'static str = "remove-account";

    const ABOUT: &'static str = "Remove selected accounts";

    fn invoke(_matches: &clap::ArgMatches) -> Result<()> {
        let budget = binding::open_budget()?;
        let accounts = budget.accounts()?;

        if accounts.is_empty() {
            //
            // Nothing to do here. It is not an error!
            //

            return Ok(());
        }

        let printable_accounts: Vec<_> = accounts
            .iter()
            .map(|account| &account.name)
            .collect();

        let selection = console::select_multiple_from_with_prompt(
            &printable_accounts, "Select accounts to remove")?;

        for idx in selection {
            let account = &accounts[idx];
            let force = console::confirm_with_prompt(
                "Remove account with all corresponding transactions?", false)?;

            match budget.remove_account(account.id.unwrap(), force) {
                Ok(_) => {},
                Err(e) => eprintln!("Cannot remove account '{}': {}", account.name, e)
            }
        }

        Ok(())
    }
}


impl CommandInternal for RemoveAccount {
    type ParsedArgs = ();

    fn parse_args(_matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        Ok(())
    }
}
