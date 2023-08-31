use libbdgt::error::{Result, Error};
use libbdgt::crypto::CryptoEngine;
use libbdgt::location::Location;
use libbdgt::{budget, config, crypto, storage, location};

use crate::errors;


/// Cryptographic engine type alias for quick engine changes.
type Engine = crypto::GpgCryptoEngine;

/// Corresponding key identifier type alias.
type KeyId = <crypto::GpgCryptoEngine as CryptoEngine>::KeyId;

/// Storage type alias for quick storage changes.
type Storage = storage::DbStorage;

/// Config type alias, that corresponds to selected engine.
type Config = config::Config<Engine>;

/// Final budget manager type. It is a main interaction point with [`libbdgt].
pub(crate) type Budget = budget::Budget<Engine, Storage>;


/// Performs initialization of the storage.
/// 
/// * `key_id` - identifier of a key used to protect data
pub(crate) fn initialize_budget(key_id: &str) -> Result<Budget> {
    //
    // Check for storage existence
    //

    let loc = location::HomeLocation::new();
    if loc.exists() {
        return Err(Error::from_message_with_extra(
            errors::ALREADY_INITIALIZED, loc.root().to_str().unwrap()));
    }
    
    //
    // Let's check key presense and validity
    //

    let id = KeyId::new(key_id);
    let engine = Engine::new()?;
    engine.lookup_key(&id)?;

    //
    // Key is present and suitable for encryption,
    // now I can create storage
    //

    let config = Config::create(&loc, &id)?;
    let storage = Storage::create(&loc)?;
    
    Budget::new(engine, storage, config)
}


/// Opens budget manager with performing of some checks.
pub(crate) fn open_budget() -> Result<Budget> {
    let loc = ensure_location()?;

    //
    // Storage root exists here, now I can just open everything
    //

    let engine = Engine::new()?;
    let config = Config::open(&loc)?;
    let storage = Storage::open(&loc)?;

    Budget::new(engine, storage, config)
}


fn ensure_location() -> Result<location::HomeLocation> {
    let loc = location::HomeLocation::new();
    if loc.exists() {
        Ok(loc)
    }
    else {
        Err(Error::from_message_with_extra(
            errors::NOT_INITIALIZED, loc.root().to_str().unwrap()))
    }
}
