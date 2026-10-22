use std::ffi::{c_int, c_uint, c_void};

use super::{CuvidPicParams, CuvidVideoFormat};

#[repr(C)]
pub(crate) struct CuvidParserParams {
    pub(crate) codec_type: c_int,
    pub(crate) max_num_decode_surfaces: c_uint,
    pub(crate) clock_rate: c_uint,
    pub(crate) error_threshold: c_uint,
    pub(crate) max_display_delay: c_uint,
    pub(crate) flags: c_uint,
    pub(crate) reserved1: [c_uint; 4],
    pub(crate) user_data: *mut c_void,
    pub(crate) sequence_callback:
        Option<unsafe extern "system" fn(*mut c_void, *mut CuvidVideoFormat) -> c_int>,
    pub(crate) decode_callback:
        Option<unsafe extern "system" fn(*mut c_void, *mut CuvidPicParams) -> c_int>,
    pub(crate) display_callback:
        Option<unsafe extern "system" fn(*mut c_void, *mut c_void) -> c_int>,
    pub(crate) operating_point_callback:
        Option<unsafe extern "system" fn(*mut c_void, *mut c_void) -> c_int>,
    pub(crate) sei_message_callback:
        Option<unsafe extern "system" fn(*mut c_void, *mut c_void) -> c_int>,
    pub(crate) reserved2: [*mut c_void; 5],
    pub(crate) ext_video_info: *mut c_void,
}
