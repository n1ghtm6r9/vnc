use std::io::Read;

use rfb::read_u8;

use super::TightDecoder;
use crate::{
    DecodeError, TIGHT_EXPLICIT_FILTER, TIGHT_FILTER_COPY, TIGHT_FILTER_GRADIENT,
    TIGHT_FILTER_PALETTE, TIGHT_STREAM_MASK,
};

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn basic(&mut self, method: u8) -> Result<(), DecodeError> {
        let stream = usize::from(method & TIGHT_STREAM_MASK);
        let filter = if method & TIGHT_EXPLICIT_FILTER != 0 {
            read_u8(self.reader)?
        } else {
            TIGHT_FILTER_COPY
        };
        match filter {
            TIGHT_FILTER_COPY => self.copy_filter(stream),
            TIGHT_FILTER_PALETTE => self.palette_filter(stream),
            TIGHT_FILTER_GRADIENT => self.gradient_filter(stream),
            other => Err(DecodeError::Corrupt(format!(
                "the Tight filter {other} is not defined"
            ))),
        }
    }
}
