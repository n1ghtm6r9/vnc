use std::io::Read;

use crate::{
    MAX_CUT_TEXT_LENGTH, ProtocolError, apple_pasteboard_text, inflate_pasteboard, read_bytes,
    read_u32, skip_bytes,
};

pub fn read_apple_clipboard(reader: &mut impl Read) -> Result<(u32, Option<String>), ProtocolError> {
    skip_bytes(reader, 3)?;
    let session_id = read_u32(reader)?;
    let uncompressed = read_u32(reader)?;
    let compressed = read_u32(reader)?;
    if uncompressed > MAX_CUT_TEXT_LENGTH || compressed > MAX_CUT_TEXT_LENGTH {
        skip_bytes(reader, compressed as usize)?;
        return Ok((session_id, None));
    }
    let data = read_bytes(reader, compressed as usize)?;
    let text = inflate_pasteboard(&data, uncompressed as usize)
        .and_then(|pasteboard| apple_pasteboard_text(&pasteboard));
    Ok((session_id, text))
}
