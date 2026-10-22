use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum AuthError {
    Io(io::Error),
    InvalidServerKey(String),
    Random(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::Io(error) => {
                write!(f, "the connection broke during authentication: {error}")
            }
            AuthError::InvalidServerKey(reason) => {
                write!(f, "the server sent a key this client cannot use: {reason}")
            }
            AuthError::Random(reason) => {
                write!(f, "the system could not provide random bytes: {reason}")
            }
        }
    }
}

impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AuthError::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for AuthError {
    fn from(error: io::Error) -> Self {
        AuthError::Io(error)
    }
}
