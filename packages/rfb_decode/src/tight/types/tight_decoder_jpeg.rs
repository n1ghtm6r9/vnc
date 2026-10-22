use std::io::Read;

use super::TightDecoder;
use crate::{
    DecodeError, TIGHT_RGB_BYTES, decode_jpeg, read_compact_length, read_compressed_payload,
    rgb24_row_to_rgba, write_rows,
};

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn jpeg(&mut self) -> Result<(), DecodeError> {
        let length = read_compact_length(self.reader)?;
        let rgb_bytes = self.rect.area() * TIGHT_RGB_BYTES;
        read_compressed_payload(
            self.reader,
            &mut self.state.compressed,
            length,
            rgb_bytes.saturating_mul(2),
        )?;
        let produced = decode_jpeg(&self.state.compressed, self.rect, &mut self.state.inflated)?;
        write_rows(
            self.framebuffer,
            self.rect,
            &self.state.inflated[..produced],
            rgb24_row_to_rgba,
        );
        Ok(())
    }
}
