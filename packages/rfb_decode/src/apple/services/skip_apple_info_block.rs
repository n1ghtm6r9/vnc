use std::io::Read;

use rfb::{read_u16, skip_bytes};

use crate::{DecodeError, RectOutcome};

pub fn skip_apple_info_block(reader: &mut impl Read) -> Result<RectOutcome, DecodeError> {
    let length = usize::from(read_u16(reader)?);
    skip_bytes(reader, length)?;
    Ok(RectOutcome::Ignored)
}
