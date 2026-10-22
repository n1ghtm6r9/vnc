use rfb::Rect;

use super::ZrleDecoder;
use crate::DecodeError;

impl ZrleDecoder<'_> {
    pub(super) fn solid_tile(&mut self, tile: Rect) -> Result<(), DecodeError> {
        let colour = self.read_pixel()?;
        self.framebuffer.fill_rect(tile, colour);
        Ok(())
    }
}
