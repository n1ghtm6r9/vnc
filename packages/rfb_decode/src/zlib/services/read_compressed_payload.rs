use std::io::{self, Read};

use super::compressed_payload_limit;
use crate::DecodeError;

pub(crate) fn read_compressed_payload(
    reader: &mut impl Read,
    buffer: &mut Vec<u8>,
    length: usize,
    inflated_limit: usize,
) -> Result<(), DecodeError> {
    if length > compressed_payload_limit(inflated_limit) {
        return Err(DecodeError::Corrupt(format!(
            "a compressed payload of {length} bytes is too large for a rect needing at most {inflated_limit} bytes"
        )));
    }
    buffer.clear();
    let wanted = u64::try_from(length).unwrap_or(u64::MAX);
    let read = reader.by_ref().take(wanted).read_to_end(buffer)?;
    if read < length {
        return Err(DecodeError::Io(io::Error::from(
            io::ErrorKind::UnexpectedEof,
        )));
    }
    Ok(())
}
