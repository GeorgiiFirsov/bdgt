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


/// Macro for implementing [`From<SomeError>`] in a beautiful way.
/// It simplifies implementing the trait for a new error type
/// to writing only one line of code.
macro_rules! implement_from_error {
    ($err_type:ty, $($err_types:ty),+ $(,)?) => {
        implement_from_error!($err_type);
        implement_from_error!($($err_types, )+);
    };
    ($err_type:ty $(,)?) => {
        impl From<$err_type> for Error {
            fn from(value: $err_type) -> Self {
                let msg = value.to_string();
                Error(libbdgt::error::Error::from_message(msg))
            }
        }
    }
}

implement_from_error!(
    anyhow::Error, 
    dialoguer::Error,
    minus::MinusError, 
    std::fmt::Error, 
    std::string::FromUtf8Error,
    pinentry::Error,
);


/// Macro for implementing [`From<SomeError>`] in a beautiful way
/// by forwarding the process directly to [`libbdgt::error::Error].
macro_rules! implement_from_error_fwd {
    ($err_type:ty, $($err_types:ty),+ $(,)?) => {
        implement_from_error_fwd!($err_type);
        implement_from_error_fwd!($($err_types, )+);
    };
    ($err_type:ty $(,)?) => {
        impl From<$err_type> for Error {
            fn from(value: $err_type) -> Self {
                Error(libbdgt::error::Error::from(value))
            }
        }
    }
}

implement_from_error_fwd!(
    std::io::Error,
);
