use super::MvsBits;
use crate::DecodeError;

impl MvsBits<'_> {
    pub(crate) fn leading_ones(&mut self) -> Result<u32, DecodeError> {
        let mut count = 0u32;
        while self.read_bit()? == 1 {
            count = count.saturating_add(1);
        }
        Ok(count)
    }
}
