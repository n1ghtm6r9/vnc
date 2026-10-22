use rfb::Rect;

use crate::{DecodeError, DecodeTarget, RectOutcome, resize_framebuffer};

pub fn decode_desktop_size(
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    resize_framebuffer(target.framebuffer, rect.width, rect.height)?;
    Ok(RectOutcome::DesktopSize {
        width: rect.width,
        height: rect.height,
    })
}
