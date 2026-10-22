use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum ProtocolError {
    Io(io::Error),
    UnsupportedVersion(String),
    ConnectionRefused(String),
    AuthenticationFailed(String),
    UnknownMessage(u8),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::Io(error) => write!(f, "the connection broke: {error}"),
            ProtocolError::UnsupportedVersion(banner) => {
                write!(f, "the server speaks a protocol this client does not know: {banner:?}")
            }
            ProtocolError::ConnectionRefused(reason) => {
                write!(f, "the server refused the connection: {reason}")
            }
            ProtocolError::AuthenticationFailed(reason) if reason.is_empty() => {
                write!(f, "the server rejected the credentials")
            }
            ProtocolError::AuthenticationFailed(reason) => {
                write!(f, "the server rejected the credentials: {reason}")
            }
            ProtocolError::UnknownMessage(message_type) => {
                write!(f, "the peer sent message type {message_type}, which cannot be read")
            }
        }
    }
}

impl Error for ProtocolError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProtocolError::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ProtocolError {
    fn from(error: io::Error) -> Self {
        ProtocolError::Io(error)
    }
}
