use std::io::Read;

use rfb::{Rect, read_u8, skip_bytes};

use crate::{
    DecodeError, DecodeTarget, EXTENDED_PADDING_BYTES, EXTENDED_REASON_CLIENT,
    EXTENDED_SCREEN_BYTES, RectOutcome, decode_desktop_size,
};

pub fn decode_extended_desktop_size(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let screens = usize::from(read_u8(reader)?);
    skip_bytes(
        reader,
        EXTENDED_PADDING_BYTES + screens * EXTENDED_SCREEN_BYTES,
    )?;
    if rect.x == EXTENDED_REASON_CLIENT && rect.y != 0 {
        log::warn!(
            "[DECODE] the server refused the resize request with status {}",
            rect.y
        );
        return Ok(RectOutcome::DesktopSize {
            width: target.framebuffer.width(),
            height: target.framebuffer.height(),
        });
    }
    decode_desktop_size(rect, target)
}
