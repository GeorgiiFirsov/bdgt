/// Command line arguments parsing error description.
pub(crate) const PARSE_ERROR: &str = "Cannot properly parse supplied arguments";

/// Error, that is occurred when initialization is performed more than once.
pub(crate) const ALREADY_INITIALIZED: &str = "Application's storage is already initialized";

/// Error, that is occurred when initialization is not performed, but other command is invoked.
pub(crate) const NOT_INITIALIZED: &str = "Application's storage is not initialized yet";

/// Out-of-range error description
pub(crate) const VALUE_OUT_OF_RANGE: &str = "Value is out of range";
