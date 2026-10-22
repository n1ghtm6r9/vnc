use std::any::Any;

use crate::DecodeError;

pub(crate) fn jpeg_decoder_panic(payload: Box<dyn Any + Send>) -> DecodeError {
    let reason = payload
        .downcast_ref::<&str>()
        .map(|text| (*text).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "no message".to_owned());
    DecodeError::Corrupt(format!(
        "the Tight JPEG made the JPEG decoder fail: {reason}"
    ))
}
