use std::ffi::{c_int, c_uint};

#[repr(C)]
pub(crate) struct CuvidVideoFormat {
    pub(crate) codec: c_int,
    pub(crate) frame_rate_numerator: c_uint,
    pub(crate) frame_rate_denominator: c_uint,
    pub(crate) progressive_sequence: u8,
    pub(crate) bit_depth_luma_minus8: u8,
    pub(crate) bit_depth_chroma_minus8: u8,
    pub(crate) min_num_decode_surfaces: u8,
    pub(crate) coded_width: c_uint,
    pub(crate) coded_height: c_uint,
    pub(crate) display_left: c_int,
    pub(crate) display_top: c_int,
    pub(crate) display_right: c_int,
    pub(crate) display_bottom: c_int,
    pub(crate) chroma_format: c_int,
    pub(crate) bitrate: c_uint,
    pub(crate) display_aspect_ratio_x: c_int,
    pub(crate) display_aspect_ratio_y: c_int,
    pub(crate) video_signal_flags: u8,
    pub(crate) colour_primaries: u8,
    pub(crate) transfer_characteristics: u8,
    pub(crate) matrix_coefficients: u8,
    pub(crate) seqhdr_data_length: c_uint,
}
