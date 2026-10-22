use super::ZrleDecoder;
use crate::DecodeError;

impl ZrleDecoder<'_> {
    pub(super) fn read_palette(&mut self, size: usize) -> Result<(), DecodeError> {
        let pixel_bytes = self.converter.compact_bytes_per_pixel();
        let bytes = self.take(size * pixel_bytes)?;
        for (slot, pixel) in self.palette.iter_mut().zip(bytes.chunks_exact(pixel_bytes)) {
            *slot = self.converter.compact_rgba(pixel);
        }
        Ok(())
    }
}
