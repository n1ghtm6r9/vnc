use std::io::Read;

use super::TightDecoder;
use crate::DecodeError;

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn fill(&mut self) -> Result<(), DecodeError> {
        let mut bytes = [0u8; 4];
        let pixel = &mut bytes[..self.converter.tight_bytes_per_pixel()];
        self.reader.read_exact(pixel)?;
        self.framebuffer
            .fill_rect(self.rect, self.converter.tight_rgba(pixel));
        Ok(())
    }
}
