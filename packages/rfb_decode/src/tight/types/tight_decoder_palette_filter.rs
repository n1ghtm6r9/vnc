use std::io::Read;

use rfb::read_u8;

use super::TightDecoder;
use crate::{DecodeError, read_scratch};

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn palette_filter(&mut self, stream: usize) -> Result<(), DecodeError> {
        let colours = usize::from(read_u8(self.reader)?) + 1;
        if colours < 2 {
            return Err(DecodeError::Corrupt(
                "a Tight palette needs at least two colours".to_owned(),
            ));
        }
        let pixel_bytes = self.converter.tight_bytes_per_pixel();
        let entries = read_scratch(self.reader, &mut self.state.wire, colours * pixel_bytes)?;
        for (slot, entry) in self
            .palette
            .iter_mut()
            .zip(entries.chunks_exact(pixel_bytes))
        {
            *slot = self.converter.tight_rgba(entry);
        }
        let width = usize::from(self.rect.width);
        if colours == 2 {
            self.fetch(stream, width.div_ceil(8) * usize::from(self.rect.height))?;
            self.paint_bits();
        } else {
            self.fetch(stream, width * usize::from(self.rect.height))?;
            self.paint_indices(colours)?;
        }
        Ok(())
    }
}
