use std::io::Read;

use rfb::read_u8;

use crate::DecodeError;

pub(crate) fn read_compact_length(reader: &mut impl Read) -> Result<usize, DecodeError> {
    let mut length = 0usize;
    for shift in [0u32, 7, 14] {
        let byte = read_u8(reader)?;
        let last = shift == 14;
        let value = if last { byte } else { byte & 0x7F };
        length |= usize::from(value) << shift;
        if last || byte & 0x80 == 0 {
            break;
        }
    }
    Ok(length)
}
