use libbdgt::crypto::CryptoEngine;
use libbdgt::location::Location;

use libbdgt::{core, crypto, config, storage, location, sync};
use libbdgt::error::{Result, Error};

use crate::errors;


/// Cryptographic engine type alias for quick engine changes.
type CryptographicEngine = crypto::GpgCryptoEngine;

/// Synchronization engine type alias for quick engine changes.
type SynchronizationEngine = sync::GitSyncEngine;

/// Corresponding key identifier type alias.
type KeyId = <CryptographicEngine as CryptoEngine>::KeyId;

/// Storage type alias for quick storage changes.
type Storage = storage::DbStorage;

/// Config type alias for quick config changes.
type Config = config::Config<CryptographicEngine>;

/// Budget type alias. Instantiation of generic type with concrete parameters.
/// Public for current crate to allow passing as a parameter into functions.
pub(crate) type Budget = core::Budget<CryptographicEngine, SynchronizationEngine, Storage>;


/// Queries for cryptographic engine information.
/// 
/// Returns engine's name and version.
pub(crate) fn query_engine_info() -> Result<(&'static str, &'static str)> {
    let engine = CryptographicEngine::new_dummy()?;
    Ok((engine.engine(), engine.version()))
}


/// Performs initialization of the storage.
/// 
/// * `key_id` - identifier of a key used to protect data
pub(crate) fn initialize_budget(key_id: &str, remote: Option<&str>) -> Result<Budget> {
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
    let engine = CryptographicEngine::create(&loc, &id)?;

    //
    // Key is present and suitable for encryption,
    // now I can create the rest things
    //

    let config = Config::create(&loc, &id)?;
    let storage = Storage::create(&loc)?;
    let sync_engine = SynchronizationEngine::create(&loc, remote)?;
    
    Budget::new(engine, sync_engine, storage, config)
}


/// Opens budget manager with performing of some checks.
pub(crate) fn open_budget() -> Result<Budget> {
    let loc = ensure_location()?;

    //
    // Storage root exists here, now I can just open everything
    //

    let engine = CryptographicEngine::open(&loc)?;
    let config = Config::open(&loc)?;
    let storage = Storage::open(&loc)?;
    let sync_engine = SynchronizationEngine::open(&loc)?;

    Budget::new(engine, sync_engine, storage, config)
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
