use rfb::Rect;

use super::ZrleDecoder;
use crate::DecodeError;

impl ZrleDecoder<'_> {
    pub(super) fn packed_palette_tile(
        &mut self,
        tile: Rect,
        size: usize,
    ) -> Result<(), DecodeError> {
        self.read_palette(size)?;
        let bits = match size {
            2 => 1,
            3..=4 => 2,
            _ => 4,
        };
        let mask = (1usize << bits) - 1;
        let row_bytes = (usize::from(tile.width) * bits).div_ceil(8);
        let packed = self.take(row_bytes * usize::from(tile.height))?;
        let palette = &self.palette[..size];
        for (row, source) in (0..tile.height).zip(packed.chunks_exact(row_bytes)) {
            let target = self.framebuffer.row_mut(tile.x, tile.y + row, tile.width);
            for (column, pixel) in target.iter_mut().enumerate() {
                let bit = column * bits;
                let index = usize::from(source[bit / 8] >> (8 - bits - bit % 8)) & mask;
                *pixel = *palette.get(index).ok_or_else(|| {
                    DecodeError::Corrupt(format!(
                        "the ZRLE palette index {index} is outside the palette of {size} colours"
                    ))
                })?;
            }
        }
        Ok(())
    }
}
