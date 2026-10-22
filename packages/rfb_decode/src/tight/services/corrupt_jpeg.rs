use zune_jpeg::errors::DecodeErrors;

use crate::DecodeError;

pub(crate) fn corrupt_jpeg(error: DecodeErrors) -> DecodeError {
    DecodeError::Corrupt(format!("the Tight JPEG is invalid: {error:?}"))
}
