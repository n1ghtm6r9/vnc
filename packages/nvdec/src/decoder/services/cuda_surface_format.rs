use std::ffi::c_int;

use crate::{ChromaLayout, SURFACE_FORMAT_NV12, SURFACE_FORMAT_YUV444};

pub(crate) fn cuda_surface_format(layout: ChromaLayout) -> c_int {
    match layout {
        ChromaLayout::Yuv444 => SURFACE_FORMAT_YUV444,
        ChromaLayout::Nv12 => SURFACE_FORMAT_NV12,
    }
}
