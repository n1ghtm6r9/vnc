use std::{error::Error, fmt, io};

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_decode::DecodeError;

#[derive(Debug)]
pub enum SessionError {
    Connect(io::Error),
    Protocol(ProtocolError),
    Auth(AuthError),
    Decode(DecodeError),
    NoSupportedSecurity(Vec<u8>),
    MissingCredentials,
    AppleSessionSelect,
    ClosedOnTheMac,
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::Connect(error) => write!(f, "the server cannot be reached: {error}"),
            SessionError::Protocol(error) => write!(f, "{error}"),
            SessionError::Auth(error) => write!(f, "{error}"),
            SessionError::Decode(error) => write!(f, "{error}"),
            SessionError::NoSupportedSecurity(offered) => write!(
                f,
                "the server offers security types {offered:?}, and this client supports none of them"
            ),
            SessionError::MissingCredentials => {
                write!(f, "the server asks for a user name, and none was given")
            }
            SessionError::AppleSessionSelect => write!(
                f,
                "the Mac asks which user session to join, and this client cannot choose one"
            ),
            SessionError::ClosedOnTheMac => {
                write!(f, "the user of the Mac ended the screen sharing session")
            }
        }
    }
}

impl Error for SessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SessionError::Connect(error) => Some(error),
            SessionError::Protocol(error) => Some(error),
            SessionError::Auth(error) => Some(error),
            SessionError::Decode(error) => Some(error),
            SessionError::NoSupportedSecurity(_)
            | SessionError::MissingCredentials
            | SessionError::AppleSessionSelect
            | SessionError::ClosedOnTheMac => None,
        }
    }
}

impl From<io::Error> for SessionError {
    fn from(error: io::Error) -> Self {
        SessionError::Protocol(ProtocolError::Io(error))
    }
}

impl From<ProtocolError> for SessionError {
    fn from(error: ProtocolError) -> Self {
        SessionError::Protocol(error)
    }
}

impl From<AuthError> for SessionError {
    fn from(error: AuthError) -> Self {
        SessionError::Auth(error)
    }
}

impl From<DecodeError> for SessionError {
    fn from(error: DecodeError) -> Self {
        SessionError::Decode(error)
    }
}
