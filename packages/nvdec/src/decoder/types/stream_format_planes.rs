use std::iter;

use super::{StreamFormat, SurfacePlane};
use crate::ChromaLayout;

impl StreamFormat {
    pub(crate) fn planes(&self) -> Vec<SurfacePlane> {
        let left = self.display.left as usize;
        let top = self.display.top as usize;
        let width = self.display.width as usize;
        let height = self.display.height as usize;
        let chroma = self.chroma.chroma_planes(width, height);
        let (chroma_left, chroma_top) = match self.chroma {
            ChromaLayout::Yuv444 => (left, top),
            ChromaLayout::Nv12 => (left & !1, top / 2),
        };
        let luma = SurfacePlane {
            index: 0,
            x_bytes: left,
            y: top,
            width_bytes: width,
            rows: height,
        };
        let chroma_planes = (1..=chroma.count).map(|index| SurfacePlane {
            index,
            x_bytes: chroma_left,
            y: chroma_top,
            width_bytes: chroma.row_bytes,
            rows: chroma.rows,
        });
        iter::once(luma).chain(chroma_planes).collect()
    }
}
