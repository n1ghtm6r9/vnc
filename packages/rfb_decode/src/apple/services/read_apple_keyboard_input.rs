use std::io::Read;

use rfb::{read_bytes, read_u16};

use crate::{DecodeError, RectOutcome, apple_input_source};

pub fn read_apple_keyboard_input(reader: &mut impl Read) -> Result<RectOutcome, DecodeError> {
    let length = usize::from(read_u16(reader)?);
    let payload = read_bytes(reader, length)?;
    if let Some(source) = apple_input_source(&payload) {
        log::info!("[DECODE] the Mac's input source is {source}");
    }
    Ok(RectOutcome::Ignored)
}
