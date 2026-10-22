use std::{mem, ptr};

use super::DecoderState;
use crate::NvdecError;

impl DecoderState {
    pub(crate) fn destroy_decoder(&mut self) -> Result<(), NvdecError> {
        self.format = None;
        let decoder = mem::replace(&mut self.decoder, ptr::null_mut());
        if decoder.is_null() {
            return Ok(());
        }
        let cuda = self.cuda;
        cuda.with_context(self.primary.context, || {
            cuda.check("cuvidDestroyDecoder", unsafe {
                (cuda.destroy_decoder)(decoder)
            })
        })
    }
}
