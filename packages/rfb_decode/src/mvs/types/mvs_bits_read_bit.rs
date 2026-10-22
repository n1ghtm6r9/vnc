use super::MvsBits;
use crate::{DecodeError, bitstream_ended};

impl MvsBits<'_> {
    pub(crate) fn read_bit(&mut self) -> Result<u32, DecodeError> {
        let byte = self
            .data
            .get(self.position / 8)
            .ok_or_else(bitstream_ended)?;
        let bit = (byte >> (7 - self.position % 8)) & 1;
        self.position += 1;
        Ok(u32::from(bit))
    }
}
