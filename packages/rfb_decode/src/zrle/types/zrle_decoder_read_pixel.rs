use super::ZrleDecoder;
use crate::DecodeError;

impl ZrleDecoder<'_> {
    pub(super) fn read_pixel(&mut self) -> Result<u32, DecodeError> {
        let bytes = self.take(self.converter.compact_bytes_per_pixel())?;
        Ok(self.converter.compact_rgba(bytes))
    }
}
