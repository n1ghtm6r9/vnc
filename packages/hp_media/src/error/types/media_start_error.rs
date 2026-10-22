use std::{error::Error, fmt, io};

use nvdec::NvdecError;

#[derive(Debug)]
pub enum MediaStartError {
    Bind(io::Error),
    Decoder(NvdecError),
    Thread(io::Error),
    Stopped,
}

impl fmt::Display for MediaStartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaStartError::Bind(error) => write!(f, "the media ports cannot be opened: {error}"),
            MediaStartError::Decoder(error) => write!(f, "the video decoder cannot start: {error}"),
            MediaStartError::Thread(error) => write!(f, "a media thread cannot start: {error}"),
            MediaStartError::Stopped => write!(f, "the media thread stopped while starting"),
        }
    }
}

impl Error for MediaStartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MediaStartError::Bind(error) | MediaStartError::Thread(error) => Some(error),
            MediaStartError::Decoder(error) => Some(error),
            MediaStartError::Stopped => None,
        }
    }
}
