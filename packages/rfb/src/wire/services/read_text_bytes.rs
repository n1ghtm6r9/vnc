use std::io::{self, Read};

use super::{read_bytes, read_u32};
use crate::MAX_TEXT_LENGTH;

pub fn read_text_bytes(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    let len = read_u32(reader)?;
    if len > MAX_TEXT_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("a text of {len} bytes is longer than this client accepts"),
        ));
    }
    read_bytes(reader, len as usize)
}
