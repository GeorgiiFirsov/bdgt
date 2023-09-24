/// Structure, that describes all errors in bdgt.
#[derive(Debug, PartialEq)]
pub(crate) struct Error(libbdgt::error::Error);


/// Crate-specific alias for [`std::result::Result`] instantiated 
/// with [`crate::error::Error`].
pub(crate) type Result<T> = std::result::Result<T, Error>;


impl Error {
    /// Constructs an error from message.
    /// 
    /// * `msg` - error message as something convertible into a [`alloc::string::String`]
    pub fn from_message<M>(msg: M) -> Self 
    where
        M: Into<String>
    {
        Error(libbdgt::error::Error::from_message(msg))
    }

    /// Constructs an error from message with some extra information.
    /// 
    /// * `msg` - error message as something convertible into a [`alloc::string::String`]
    /// * `extra` - extra information as something convertible into a [`alloc::string::String`]
    pub fn from_message_with_extra<M, E>(msg: M, extra: E) -> Self
    where
        M: Into<String>,
        E: Into<String>
    {
        Error(libbdgt::error::Error::from_message_with_extra(msg, extra))
    }
}


impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl std::ops::Deref for Error {
    type Target = libbdgt::error::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl From<libbdgt::error::Error> for Error {
    fn from(value: libbdgt::error::Error) -> Self {
        Error(value)
    }
}


impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error(libbdgt::error::Error::from(value))
    }
}


impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        let msg = value.to_string();
        Error(libbdgt::error::Error::from_message(msg))
    }
}
