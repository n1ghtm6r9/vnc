use std::io::Cursor;

use plist::Value;

use crate::{KEY_MEDIA_BLOB, MediaError, ProtoField, decode_proto, zlib_inflate};

pub(crate) fn read_media_blob(bytes: &[u8]) -> Result<Vec<ProtoField>, MediaError> {
    let offer = Value::from_reader(Cursor::new(bytes))
        .map_err(|error| MediaError::Plist(error.to_string()))?;
    let blob = offer
        .as_dictionary()
        .and_then(|offer| offer.get(KEY_MEDIA_BLOB))
        .and_then(Value::as_data)
        .ok_or_else(|| MediaError::Malformed("media answer without a media blob".into()))?;
    decode_proto(&zlib_inflate(blob)?)
}
