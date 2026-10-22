use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformError {
    Os(String),
}

impl fmt::Display for PlatformError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformError::Os(message) => write!(formatter, "the operating system refused the request: {message}"),
        }
    }
}

impl Error for PlatformError {}

#[cfg(windows)]
impl From<windows::core::Error> for PlatformError {
    fn from(error: windows::core::Error) -> PlatformError {
        PlatformError::Os(error.message())
    }
}
