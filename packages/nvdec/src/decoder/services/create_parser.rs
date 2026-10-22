use std::ptr;

use super::{handle_picture, handle_sequence};
use crate::{
    CODEC_HEVC, CuVideoParser, CudaFunctions, CuvidParserParams, DecoderState, NvdecError,
};

pub(crate) fn create_parser(
    cuda: &CudaFunctions,
    state: *mut DecoderState,
) -> Result<CuVideoParser, NvdecError> {
    let mut params = CuvidParserParams {
        codec_type: CODEC_HEVC,
        max_num_decode_surfaces: 1,
        clock_rate: 0,
        error_threshold: 0,
        max_display_delay: 0,
        flags: 0,
        reserved1: [0; 4],
        user_data: state.cast(),
        sequence_callback: Some(handle_sequence),
        decode_callback: Some(handle_picture),
        display_callback: None,
        operating_point_callback: None,
        sei_message_callback: None,
        reserved2: [ptr::null_mut(); 5],
        ext_video_info: ptr::null_mut(),
    };
    let mut parser = ptr::null_mut();
    cuda.check("cuvidCreateVideoParser", unsafe {
        (cuda.create_video_parser)(&mut parser, &mut params)
    })?;
    Ok(parser)
}
