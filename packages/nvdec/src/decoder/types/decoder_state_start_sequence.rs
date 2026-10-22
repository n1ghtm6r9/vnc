use std::ffi::c_int;

use super::{DecoderState, StreamFormat};
use crate::{CuvidVideoFormat, NvdecError, VIDEO_FULL_RANGE_FLAG};

impl DecoderState {
    pub(crate) fn start_sequence(&mut self, video: &CuvidVideoFormat) -> Result<c_int, NvdecError> {
        let format = StreamFormat::read(video)?;
        let reusable = !self.decoder.is_null()
            && self
                .format
                .is_some_and(|current| current.same_surfaces(&format));
        if !reusable {
            self.destroy_decoder()?;
            self.create_decoder(&format)?;
            log::info!(
                "[DECODE] the stream signals colour primaries {}, transfer {}, matrix {}, full range {}",
                video.colour_primaries,
                video.transfer_characteristics,
                video.matrix_coefficients,
                video.video_signal_flags & VIDEO_FULL_RANGE_FLAG != 0
            );
        }
        self.format = Some(format);
        Ok(c_int::from(format.decode_surfaces))
    }
}
