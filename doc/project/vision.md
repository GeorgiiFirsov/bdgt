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
