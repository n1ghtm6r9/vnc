use std::io::Read;

use super::TightDecoder;
use crate::{DecodeError, write_rows};

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn copy_filter(&mut self, stream: usize) -> Result<(), DecodeError> {
        let expected = self.rect.area() * self.converter.tight_bytes_per_pixel();
        self.fetch(stream, expected)?;
        let converter = self.converter;
        write_rows(
            self.framebuffer,
            self.rect,
            &self.state.inflated[..expected],
            |source, target| converter.tight_rgba_row(source, target),
        );
        Ok(())
    }
}
