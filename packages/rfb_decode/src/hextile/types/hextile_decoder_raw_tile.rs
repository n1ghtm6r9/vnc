use std::io::Read;

use rfb::Rect;

use super::HextileDecoder;
use crate::{DecodeError, read_scratch, write_raw_rows};

impl<R: Read> HextileDecoder<'_, R> {
    pub(super) fn raw_tile(&mut self, tile: Rect) -> Result<(), DecodeError> {
        let length = tile.area() * self.converter.bytes_per_pixel();
        let data = read_scratch(self.reader, &mut self.state.wire, length)?;
        write_raw_rows(self.converter, self.framebuffer, tile, data);
        Ok(())
    }
}
