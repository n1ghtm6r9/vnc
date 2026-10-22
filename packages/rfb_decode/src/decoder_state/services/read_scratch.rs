use std::io::Read;

use crate::DecodeError;

pub(crate) fn read_scratch<'a>(
    reader: &mut impl Read,
    buffer: &'a mut Vec<u8>,
    len: usize,
) -> Result<&'a [u8], DecodeError> {
    if buffer.len() < len {
        buffer.resize(len, 0);
    }
    let filled = &mut buffer[..len];
    reader.read_exact(filled)?;
    Ok(filled)
}
