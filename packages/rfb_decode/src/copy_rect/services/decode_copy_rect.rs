use std::io::Read;

use rfb::{Rect, read_u16};

use crate::{DecodeError, DecodeTarget, RectOutcome, ensure_rect_fits};

pub fn decode_copy_rect(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let DecodeTarget { framebuffer, .. } = target;
    ensure_rect_fits(framebuffer, rect)?;
    let src_x = read_u16(reader)?;
    let src_y = read_u16(reader)?;
    ensure_rect_fits(
        framebuffer,
        Rect::new(src_x, src_y, rect.width, rect.height),
    )?;
    framebuffer.copy_rect(src_x, src_y, rect);
    Ok(RectOutcome::Pixels(rect))
}
