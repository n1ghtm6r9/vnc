use std::ffi::c_int;

use super::{DecodedFrame, DecoderState, StreamFormat};
use crate::{CuvidProcParams, NvdecError};

impl DecoderState {
    pub(crate) fn output_picture(
        &mut self,
        index: c_int,
        format: &StreamFormat,
    ) -> Result<(), NvdecError> {
        let cuda = self.cuda;
        let mut params = CuvidProcParams::progressive();
        let mut frame = 0;
        let mut pitch = 0;
        cuda.check("cuvidMapVideoFrame64", unsafe {
            (cuda.map_video_frame)(self.decoder, index, &mut frame, &mut pitch, &mut params)
        })?;
        let copied = self.copy_planes(frame, pitch as usize, format);
        let unmapped = cuda.check("cuvidUnmapVideoFrame64", unsafe {
            (cuda.unmap_video_frame)(self.decoder, frame)
        });
        let planes = copied?;
        unmapped?;
        self.frames.push(DecodedFrame {
            width: format.display.width,
            height: format.display.height,
            layout: format.chroma,
            colour: format.colour,
            planes,
        });
        Ok(())
    }
}
