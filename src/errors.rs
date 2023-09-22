/// Command line arguments parsing error description.
pub(crate) const PARSE_ERROR: &str = "Cannot properly parse supplied arguments";

/// Error, that is occurred when initialization is performed more than once.
pub(crate) const ALREADY_INITIALIZED: &str = "Application's storage is already initialized";

/// Error, that is occurred when initialization is not performed, but other command is invoked.
pub(crate) const NOT_INITIALIZED: &str = "Application's storage is not initialized yet";

/// Out-of-range error description
pub(crate) const VALUE_OUT_OF_RANGE: &str = "Value is out of range";

/// No accounts present, when at least one is required
pub(crate) const NO_ACCOUNTS: &str = "No accounts exist yet";

/// No categories present, when at least one is required
pub(crate) const NO_CATEGORIES: &str = "No categories exist yet";
