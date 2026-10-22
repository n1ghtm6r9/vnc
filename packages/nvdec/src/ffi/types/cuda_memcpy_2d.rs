use std::ffi::{c_uint, c_void};

use super::CuDevicePtr;

#[repr(C)]
pub(crate) struct CudaMemcpy2d {
    pub(crate) src_x_in_bytes: usize,
    pub(crate) src_y: usize,
    pub(crate) src_memory_type: c_uint,
    pub(crate) src_host: *const c_void,
    pub(crate) src_device: CuDevicePtr,
    pub(crate) src_array: *mut c_void,
    pub(crate) src_pitch: usize,
    pub(crate) dst_x_in_bytes: usize,
    pub(crate) dst_y: usize,
    pub(crate) dst_memory_type: c_uint,
    pub(crate) dst_host: *mut c_void,
    pub(crate) dst_device: CuDevicePtr,
    pub(crate) dst_array: *mut c_void,
    pub(crate) dst_pitch: usize,
    pub(crate) width_in_bytes: usize,
    pub(crate) height: usize,
}
