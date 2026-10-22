use rfb::Rect;

use crate::{DecodeError, Framebuffer};

pub(crate) fn ensure_rect_fits(framebuffer: &Framebuffer, rect: Rect) -> Result<(), DecodeError> {
    if rect.fits_within(framebuffer.width(), framebuffer.height()) {
        Ok(())
    } else {
        Err(DecodeError::RectOutOfBounds(rect))
    }
}
