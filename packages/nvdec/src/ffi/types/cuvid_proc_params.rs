use std::ffi::{c_int, c_uint, c_void};

use super::CuDevicePtr;

#[repr(C)]
pub(crate) struct CuvidProcParams {
    pub(crate) progressive_frame: c_int,
    pub(crate) second_field: c_int,
    pub(crate) top_field_first: c_int,
    pub(crate) unpaired_field: c_int,
    pub(crate) reserved_flags: c_uint,
    pub(crate) reserved_zero: c_uint,
    pub(crate) raw_input_dptr: CuDevicePtr,
    pub(crate) raw_input_pitch: c_uint,
    pub(crate) raw_input_format: c_uint,
    pub(crate) raw_output_dptr: CuDevicePtr,
    pub(crate) raw_output_pitch: c_uint,
    pub(crate) reserved1: c_uint,
    pub(crate) output_stream: *mut c_void,
    pub(crate) reserved: [c_uint; 46],
    pub(crate) histogram_dptr: *mut CuDevicePtr,
    pub(crate) proc_ext: *mut c_void,
}
