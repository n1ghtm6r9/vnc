use std::{error::Error, fmt, io};

use rfb::Rect;

#[derive(Debug)]
pub enum DecodeError {
    Io(io::Error),
    UnsupportedEncoding(i32),
    RectOutOfBounds(Rect),
    Corrupt(String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Io(error) => {
                write!(
                    f,
                    "the connection broke while a rectangle was being read: {error}"
                )
            }
            DecodeError::UnsupportedEncoding(encoding) => {
                write!(
                    f,
                    "the server sent a rectangle with encoding {encoding}, which this client cannot decode"
                )
            }
            DecodeError::RectOutOfBounds(rect) => write!(
                f,
                "the server sent a {}x{} rectangle at ({}, {}) that does not fit the screen",
                rect.width, rect.height, rect.x, rect.y
            ),
            DecodeError::Corrupt(reason) => {
                write!(
                    f,
                    "the server sent a rectangle that cannot be decoded: {reason}"
                )
            }
        }
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DecodeError::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        DecodeError::Io(error)
    }
}
