# Vision for `bdgt` app

This file contains informal description of "requirements" for `bdgt` app.

## Scenarios

0. **Querying for engine information**. User may request information about cryptographic
engine. App should return name and version of current engine.

1. **Initialization**. At this step user performs an initialization of storage for its data.
User must specify an encryption key identifier in engine-specific format.

2. **Adding account**. User provides the following information about a new account: 
user-friendly name and a current balance (defaults to 0). Account information is written
into DB in encrypted form.

3. **Account removal**. User selects several accounts to remove. For each account app
asks if one want to remove all corresponding transactions too. If one agrees with
transactions removal, then account will be removed anyway. Otherwise, it will be
removed if and only if no corresponsing transactions exist.

4. **Adding category**. User provides category type (income/outcome/...) and its name.
Multiple categories with the same name can be created.

5. **Category removal**. User selects several categories to remove. Category can be
removed if and only if no transactions with the category exist. Failing to remove
one category will not break other categories removal.

6. **Adding transaction**. User selects a category and account, enters description 
and amount, optionally enters a date and time. Afterwards, correct sign is assigned
to the amount depending on category. 
