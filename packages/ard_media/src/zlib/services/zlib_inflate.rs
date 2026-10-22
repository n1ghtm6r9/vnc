use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::{MAX_INFLATED_BYTES, MediaError};

pub(crate) fn zlib_inflate(bytes: &[u8]) -> Result<Vec<u8>, MediaError> {
    let mut inflated = Vec::new();
    ZlibDecoder::new(bytes)
        .take(MAX_INFLATED_BYTES + 1)
        .read_to_end(&mut inflated)
        .map_err(MediaError::Compression)?;
    if inflated.len() as u64 > MAX_INFLATED_BYTES {
        return Err(MediaError::Malformed(
            "media blob that unpacks too large".into(),
        ));
    }
    Ok(inflated)
}
