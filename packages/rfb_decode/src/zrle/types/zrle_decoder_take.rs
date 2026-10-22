use super::ZrleDecoder;
use crate::{DecodeError, truncated_tile};

impl<'a> ZrleDecoder<'a> {
    pub(super) fn take(&mut self, length: usize) -> Result<&'a [u8], DecodeError> {
        let data: &'a [u8] = self.data;
        let end = self
            .position
            .checked_add(length)
            .ok_or_else(truncated_tile)?;
        let taken = data.get(self.position..end).ok_or_else(truncated_tile)?;
        self.position = end;
        Ok(taken)
    }
}
