use std::io::Read;

use rfb::{Rect, read_u32};

use crate::{
    DecodeError, DecodeTarget, FIRST_CUSTOM_APPLE_CURSOR_ID, MAX_APPLE_CURSOR_BYTES,
    MAX_CACHED_APPLE_CURSORS, RectOutcome, SystemCursor, apple_cursor_shape, read_scratch,
};

pub fn decode_apple_cursor(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let state = target.state;
    let id = read_u32(reader)?;
    let size = usize::try_from(read_u32(reader)?).unwrap_or(usize::MAX);
    if size > MAX_APPLE_CURSOR_BYTES {
        return Err(DecodeError::Corrupt(format!(
            "a cursor of {size} bytes is larger than {MAX_APPLE_CURSOR_BYTES} bytes"
        )));
    }
    let data = read_scratch(reader, &mut state.wire, size)?;
    if id < FIRST_CUSTOM_APPLE_CURSOR_ID {
        return Ok(RectOutcome::SystemCursor(SystemCursor::from_apple_id(id)));
    }
    if size == 0 {
        return Ok(match state.apple_cursors.get(&id) {
            Some(shape) => RectOutcome::Cursor(shape.clone()),
            None => {
                log::warn!(
                    "[DECODE] the server reused cursor {id} without sending it first, the cursor stays as it was"
                );
                RectOutcome::Ignored
            }
        });
    }
    let Some(shape) = apple_cursor_shape(data, rect) else {
        log::warn!(
            "[DECODE] cursor {id} of {}x{} cannot be decoded, the cursor stays as it was",
            rect.width,
            rect.height
        );
        return Ok(RectOutcome::Ignored);
    };
    log::info!(
        "[DECODE] the Mac cursor {id} is {}x{} with its hotspot at {},{}",
        shape.width,
        shape.height,
        shape.hotspot_x,
        shape.hotspot_y
    );
    if state.apple_cursors.len() >= MAX_CACHED_APPLE_CURSORS {
        state.apple_cursors.clear();
    }
    state.apple_cursors.insert(id, shape.clone());
    Ok(RectOutcome::Cursor(shape))
}
