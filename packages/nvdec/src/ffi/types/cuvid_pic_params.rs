use std::ffi::c_int;

#[repr(C)]
pub(crate) struct CuvidPicParams {
    pub(crate) pic_width_in_mbs: c_int,
    pub(crate) frame_height_in_mbs: c_int,
    pub(crate) curr_pic_idx: c_int,
}
