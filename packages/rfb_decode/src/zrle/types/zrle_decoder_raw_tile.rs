use rfb::Rect;

use super::ZrleDecoder;
use crate::{DecodeError, write_rows};

impl ZrleDecoder<'_> {
    pub(super) fn raw_tile(&mut self, tile: Rect) -> Result<(), DecodeError> {
        let row_bytes = usize::from(tile.width) * self.converter.compact_bytes_per_pixel();
        let data = self.take(row_bytes * usize::from(tile.height))?;
        let converter = self.converter;
        write_rows(self.framebuffer, tile, data, |source, target| {
            converter.compact_rgba_row(source, target)
        });
        Ok(())
    }
}
