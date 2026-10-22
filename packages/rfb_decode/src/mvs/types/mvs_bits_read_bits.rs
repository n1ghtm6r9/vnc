use super::MvsBits;
use crate::DecodeError;

impl MvsBits<'_> {
    pub(crate) fn read_bits(&mut self, count: u32) -> Result<u32, DecodeError> {
        let mut value = 0;
        for _ in 0..count {
            value = (value << 1) | self.read_bit()?;
        }
        Ok(value)
    }
}
