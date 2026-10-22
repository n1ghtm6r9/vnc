use std::ffi::{c_int, c_uint};

#[repr(C)]
pub(crate) struct CuvidDecodeCaps {
    pub(crate) codec_type: c_int,
    pub(crate) chroma_format: c_int,
    pub(crate) bit_depth_minus8: c_uint,
    pub(crate) reserved1: [c_uint; 3],
    pub(crate) is_supported: u8,
    pub(crate) num_nvdecs: u8,
    pub(crate) output_format_mask: u16,
    pub(crate) max_width: c_uint,
    pub(crate) max_height: c_uint,
    pub(crate) max_mb_count: c_uint,
    pub(crate) min_width: u16,
    pub(crate) min_height: u16,
    pub(crate) is_histogram_supported: u8,
    pub(crate) counter_bit_depth: u8,
    pub(crate) max_histogram_bins: u16,
    pub(crate) is_decode_stats_supported: u8,
    pub(crate) reserved4: [u8; 3],
    pub(crate) reserved3: [c_uint; 9],
}
