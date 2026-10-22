use std::io::Read;

use super::HextileDecoder;
use crate::DecodeError;

impl<R: Read> HextileDecoder<'_, R> {
    pub(super) fn read_pixel(&mut self) -> Result<u32, DecodeError> {
        let mut buffer = [0u8; 4];
        let pixel = &mut buffer[..self.converter.bytes_per_pixel()];
        self.reader.read_exact(pixel)?;
        Ok(self.converter.rgba(pixel))
    }
}
