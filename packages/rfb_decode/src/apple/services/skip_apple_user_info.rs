use std::io::Read;

use rfb::{read_u16, read_u32, skip_bytes};

use crate::{DecodeError, RectOutcome};

pub fn skip_apple_user_info(reader: &mut impl Read) -> Result<RectOutcome, DecodeError> {
    let name_length = usize::from(read_u16(reader)?);
    skip_bytes(reader, name_length)?;
    let picture_length = usize::try_from(read_u32(reader)?).unwrap_or(usize::MAX);
    skip_bytes(reader, 4)?;
    skip_bytes(reader, picture_length)?;
    Ok(RectOutcome::Ignored)
}
