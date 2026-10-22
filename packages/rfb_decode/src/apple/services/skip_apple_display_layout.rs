use std::io::Read;

use rfb::{read_u16, skip_bytes};

use crate::{APPLE_DISPLAY_LAYOUT_ENTRY_BYTES, DecodeError, RectOutcome};

pub fn skip_apple_display_layout(reader: &mut impl Read) -> Result<RectOutcome, DecodeError> {
    skip_bytes(reader, 4)?;
    let count = usize::from(read_u16(reader)?);
    skip_bytes(reader, 2 + count * APPLE_DISPLAY_LAYOUT_ENTRY_BYTES)?;
    Ok(RectOutcome::Ignored)
}
