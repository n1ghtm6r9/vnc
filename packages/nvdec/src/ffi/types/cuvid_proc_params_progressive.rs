use std::ptr;

use super::CuvidProcParams;

impl CuvidProcParams {
    pub(crate) fn progressive() -> Self {
        Self {
            progressive_frame: 1,
            second_field: 1,
            top_field_first: 1,
            unpaired_field: 0,
            reserved_flags: 0,
            reserved_zero: 0,
            raw_input_dptr: 0,
            raw_input_pitch: 0,
            raw_input_format: 0,
            raw_output_dptr: 0,
            raw_output_pitch: 0,
            reserved1: 0,
            output_stream: ptr::null_mut(),
            reserved: [0; 46],
            histogram_dptr: ptr::null_mut(),
            proc_ext: ptr::null_mut(),
        }
    }
}
