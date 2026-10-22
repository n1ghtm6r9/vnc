use std::io::Read;

use rfb::Rect;

use crate::{
    CursorShape, DecodeError, DecodeTarget, MAX_CURSOR_SIDE, RectOutcome, cursor_rgba, read_scratch,
};

pub fn decode_cursor(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let DecodeTarget {
        converter, state, ..
    } = target;
    if rect.width > MAX_CURSOR_SIDE || rect.height > MAX_CURSOR_SIDE {
        return Err(DecodeError::Corrupt(format!(
            "a cursor of {}x{} is larger than {MAX_CURSOR_SIDE}x{MAX_CURSOR_SIDE}",
            rect.width, rect.height
        )));
    }
    let (width, height) = (usize::from(rect.width), usize::from(rect.height));
    let rgba = if width == 0 || height == 0 {
        Vec::new()
    } else {
        let length = width * height * converter.bytes_per_pixel() + width.div_ceil(8) * height;
        let data = read_scratch(reader, &mut state.wire, length)?;
        cursor_rgba(converter, data, width, height)
    };
    Ok(RectOutcome::Cursor(CursorShape {
        width: rect.width,
        height: rect.height,
        hotspot_x: rect.x,
        hotspot_y: rect.y,
        rgba,
    }))
}
