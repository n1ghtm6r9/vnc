use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum MediaError {
    Truncated(&'static str),
    Malformed(String),
    Plist(String),
    Compression(io::Error),
    Random(String),
    OfferTooLarge(usize),
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaError::Truncated(what) => write!(f, "the Mac sent a truncated {what}"),
            MediaError::Malformed(reason) => write!(f, "the Mac sent a malformed {reason}"),
            MediaError::Plist(reason) => write!(f, "a media offer could not be read: {reason}"),
            MediaError::Compression(error) => {
                write!(f, "a media blob could not be unpacked: {error}")
            }
            MediaError::Random(reason) => {
                write!(f, "the system could not provide random bytes: {reason}")
            }
            MediaError::OfferTooLarge(size) => {
                write!(
                    f,
                    "a media offer of {size} bytes does not fit in one message"
                )
            }
        }
    }
}

impl Error for MediaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MediaError::Compression(error) => Some(error),
            _ => None,
        }
    }
}
