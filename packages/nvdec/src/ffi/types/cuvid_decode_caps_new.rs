use std::ffi::{c_int, c_uint};

use super::CuvidDecodeCaps;

impl CuvidDecodeCaps {
    pub(crate) fn new(codec_type: c_int, chroma_format: c_int, bit_depth_minus8: c_uint) -> Self {
        Self {
            codec_type,
            chroma_format,
            bit_depth_minus8,
            reserved1: [0; 3],
            is_supported: 0,
            num_nvdecs: 0,
            output_format_mask: 0,
            max_width: 0,
            max_height: 0,
            max_mb_count: 0,
            min_width: 0,
            min_height: 0,
            is_histogram_supported: 0,
            counter_bit_depth: 0,
            max_histogram_bins: 0,
            is_decode_stats_supported: 0,
            reserved4: [0; 3],
            reserved3: [0; 9],
        }
    }
}
