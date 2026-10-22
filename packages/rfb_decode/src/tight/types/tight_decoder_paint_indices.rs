use std::io::Read;

use super::TightDecoder;
use crate::DecodeError;

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn paint_indices(&mut self, colours: usize) -> Result<(), DecodeError> {
        let row_bytes = usize::from(self.rect.width);
        if row_bytes == 0 {
            return Ok(());
        }
        let palette = &self.palette[..colours];
        let data = &self.state.inflated[..row_bytes * usize::from(self.rect.height)];
        for (row, source) in (0..self.rect.height).zip(data.chunks_exact(row_bytes)) {
            let target = self
                .framebuffer
                .row_mut(self.rect.x, self.rect.y + row, self.rect.width);
            for (pixel, &index) in target.iter_mut().zip(source) {
                *pixel = *palette
                    .get(usize::from(index))
                    .ok_or_else(|| DecodeError::Corrupt(format!("the Tight palette index {index} is outside the palette of {colours} colours")))?;
            }
        }
        Ok(())
    }
}
