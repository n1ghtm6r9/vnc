use super::ZrleDecoder;
use crate::{DecodeError, ZRLE_RUN_CONTINUES};

impl ZrleDecoder<'_> {
    pub(super) fn read_run_length(&mut self) -> Result<usize, DecodeError> {
        let mut length = 1usize;
        loop {
            let byte = self.take_byte()?;
            length += usize::from(byte);
            if byte != ZRLE_RUN_CONTINUES {
                return Ok(length);
            }
        }
    }
}
