use std::ffi::c_int;

use crate::{CHROMA_FORMAT_420, CHROMA_FORMAT_444, ChromaLayout};

pub(crate) fn cuda_chroma_format(layout: ChromaLayout) -> c_int {
    match layout {
        ChromaLayout::Yuv444 => CHROMA_FORMAT_444,
        ChromaLayout::Nv12 => CHROMA_FORMAT_420,
    }
}
