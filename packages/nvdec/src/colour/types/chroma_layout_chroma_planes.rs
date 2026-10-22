use super::{ChromaLayout, ChromaPlanes};

impl ChromaLayout {
    pub(crate) fn chroma_planes(self, width: usize, height: usize) -> ChromaPlanes {
        match self {
            ChromaLayout::Yuv444 => ChromaPlanes {
                count: 2,
                row_bytes: width,
                rows: height,
            },
            ChromaLayout::Nv12 => ChromaPlanes {
                count: 1,
                row_bytes: width.div_ceil(2) * 2,
                rows: height.div_ceil(2),
            },
        }
    }
}
