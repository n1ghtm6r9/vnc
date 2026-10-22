use std::io::Read;

use rfb::read_u8;

use super::TightDecoder;
use crate::{DecodeError, TIGHT_FILL, TIGHT_JPEG, TIGHT_LAST_BASIC_METHOD, TIGHT_RESET_MASK};

impl<R: Read> TightDecoder<'_, R> {
    pub(crate) fn run(&mut self) -> Result<(), DecodeError> {
        let control = read_u8(self.reader)?;
        self.reset_streams(control & TIGHT_RESET_MASK);
        match control >> 4 {
            TIGHT_FILL => self.fill(),
            TIGHT_JPEG => self.jpeg(),
            method @ 0..=TIGHT_LAST_BASIC_METHOD => self.basic(method),
            method => Err(DecodeError::Corrupt(format!(
                "the Tight compression method {method} is not supported"
            ))),
        }
    }
}
