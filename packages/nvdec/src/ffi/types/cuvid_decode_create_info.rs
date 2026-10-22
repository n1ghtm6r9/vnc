use std::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub(crate) struct CuvidDecodeCreateInfo {
    pub(crate) width: c_ulong,
    pub(crate) height: c_ulong,
    pub(crate) num_decode_surfaces: c_ulong,
    pub(crate) codec_type: c_int,
    pub(crate) chroma_format: c_int,
    pub(crate) creation_flags: c_ulong,
    pub(crate) bit_depth_minus8: c_ulong,
    pub(crate) intra_decode_only: c_ulong,
    pub(crate) max_width: c_ulong,
    pub(crate) max_height: c_ulong,
    pub(crate) reserved1: c_ulong,
    pub(crate) display_area: [i16; 4],
    pub(crate) output_format: c_int,
    pub(crate) deinterlace_mode: c_int,
    pub(crate) target_width: c_ulong,
    pub(crate) target_height: c_ulong,
    pub(crate) num_output_surfaces: c_ulong,
    pub(crate) vid_lock: *mut c_void,
    pub(crate) target_rect: [i16; 4],
    pub(crate) enable_histogram: c_ulong,
    pub(crate) enable_decode_features: c_ulong,
    pub(crate) reserved2: [c_ulong; 3],
}
