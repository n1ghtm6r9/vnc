use std::io::Read;

use super::TightDecoder;
use crate::{DecodeError, TIGHT_RGB_BYTES, undo_gradient, write_rows};

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn gradient_filter(&mut self, stream: usize) -> Result<(), DecodeError> {
        if !self.converter.tight_is_rgb24() {
            return Err(DecodeError::Corrupt(
                "the Tight gradient filter needs 24 bit colour pixels".to_owned(),
            ));
        }
        let expected = self.rect.area() * TIGHT_RGB_BYTES;
        self.fetch(stream, expected)?;
        undo_gradient(
            &mut self.state.inflated[..expected],
            usize::from(self.rect.width),
        );
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
