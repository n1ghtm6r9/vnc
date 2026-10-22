use super::ZrleDecoder;
use crate::{DecodeError, truncated_tile};

impl ZrleDecoder<'_> {
    pub(super) fn take_byte(&mut self) -> Result<u8, DecodeError> {
        let byte = *self.data.get(self.position).ok_or_else(truncated_tile)?;
        self.position += 1;
        Ok(byte)
    }
}
