use std::ops::Range;

use rfb::Rect;

use super::ZrleDecoder;

impl ZrleDecoder<'_> {
    pub(super) fn paint_run(&mut self, tile: Rect, pixels: Range<usize>, colour: u32) {
        let width = usize::from(tile.width);
        let to_u16 = |value: usize| u16::try_from(value).unwrap_or(u16::MAX);
        let mut row = pixels.start / width;
        let mut column = pixels.start % width;
        let mut left = pixels.len();
        while left > 0 {
            let count = left.min(width - column);
            let x = tile.x.saturating_add(to_u16(column));
            let y = tile.y.saturating_add(to_u16(row));
            self.framebuffer.row_mut(x, y, to_u16(count)).fill(colour);
            left -= count;
            row += 1;
            column = 0;
        }
    }
}
