use std::ffi::c_int;

use super::DecoderState;
use crate::{CuvidPicParams, NvdecError};

impl DecoderState {
    pub(crate) fn decode_picture(
        &mut self,
        picture: *mut CuvidPicParams,
        index: c_int,
    ) -> Result<c_int, NvdecError> {
        let Some(format) = self.format.filter(|_| !self.decoder.is_null()) else {
            return Err(NvdecError::Unsupported(String::from(
                "a picture without a usable sequence header",
            )));
        };
        let cuda = self.cuda;
        cuda.check("cuvidDecodePicture", unsafe {
            (cuda.decode_picture)(self.decoder, picture)
        })?;
        self.output_picture(index, &format)?;
        Ok(1)
    }
}
