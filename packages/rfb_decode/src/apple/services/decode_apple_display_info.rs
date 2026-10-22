use std::io::Read;

use rfb::{Rect, apple_display_geometry, apple_display_pixel_size, read_u16};

use crate::{DecodeError, DecodeTarget, RectOutcome, read_scratch, resize_framebuffer};

pub fn decode_apple_display_info(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let length = usize::from(read_u16(reader)?);
    let payload = read_scratch(reader, &mut target.state.wire, length)?;
    let geometry = apple_display_geometry(payload);
    let (width, height) = if rect.width > 0 && rect.height > 0 {
        (rect.width, rect.height)
    } else {
        apple_display_pixel_size(payload)
            .filter(|&(width, height)| width > 0 && height > 0)
            .unwrap_or((target.framebuffer.width(), target.framebuffer.height()))
    };
    resize_framebuffer(target.framebuffer, width, height)?;
    Ok(RectOutcome::AppleDisplay {
        width,
        height,
        geometry,
    })
}
