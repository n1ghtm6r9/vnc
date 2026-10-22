use std::io::Read;

use super::TightDecoder;

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn paint_bits(&mut self) {
        let row_bytes = usize::from(self.rect.width).div_ceil(8);
        if row_bytes == 0 {
            return;
        }
        let data = &self.state.inflated[..row_bytes * usize::from(self.rect.height)];
        for (row, source) in (0..self.rect.height).zip(data.chunks_exact(row_bytes)) {
            let target = self
                .framebuffer
                .row_mut(self.rect.x, self.rect.y + row, self.rect.width);
            for (column, pixel) in target.iter_mut().enumerate() {
                let bit = (source[column / 8] >> (7 - column % 8)) & 1;
                *pixel = self.palette[usize::from(bit)];
            }
        }
    }
}
