use std::{error::Error, fmt, io, time::Duration};

use rfb_session::SessionError;

#[derive(Debug)]
pub(crate) enum SnapshotError {
    Session(SessionError),
    Ended,
    NoFrame(Duration),
    File(io::Error),
    Png(png::EncodingError),
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnapshotError::Session(error) => write!(f, "the session ended: {error}"),
            SnapshotError::Ended => {
                write!(f, "the session ended before a frame and its stats arrived")
            }
            SnapshotError::NoFrame(waited) => write!(f, "no frame arrived within {waited:?}"),
            SnapshotError::File(error) => write!(f, "the picture cannot be written: {error}"),
            SnapshotError::Png(error) => write!(f, "the picture cannot be encoded: {error}"),
        }
    }
}

impl Error for SnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SnapshotError::Session(error) => Some(error),
            SnapshotError::File(error) => Some(error),
            SnapshotError::Png(error) => Some(error),
            SnapshotError::Ended | SnapshotError::NoFrame(_) => None,
        }
    }
}

impl From<SessionError> for SnapshotError {
    fn from(error: SessionError) -> SnapshotError {
        SnapshotError::Session(error)
    }
}

impl From<io::Error> for SnapshotError {
    fn from(error: io::Error) -> SnapshotError {
        SnapshotError::File(error)
    }
}

impl From<png::EncodingError> for SnapshotError {
    fn from(error: png::EncodingError) -> SnapshotError {
        SnapshotError::Png(error)
    }
}
