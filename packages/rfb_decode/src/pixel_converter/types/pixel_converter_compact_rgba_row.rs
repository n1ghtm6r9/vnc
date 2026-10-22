use super::{CompactLayout, PixelConverter};
use crate::rgb24_row_to_rgba;

impl PixelConverter {
    pub(crate) fn compact_rgba_row(&self, src: &[u8], dst: &mut [u32]) {
        match self.compact {
            CompactLayout::Full => self.rgba_row(src, dst),
            CompactLayout::LeastSignificantThree if self.rgba32_layout => {
                rgb24_row_to_rgba(src, dst)
            }
            CompactLayout::LeastSignificantThree | CompactLayout::MostSignificantThree => {
                for (pixel, bytes) in dst.iter_mut().zip(src.as_chunks::<3>().0) {
                    *pixel = self.compact_rgba(bytes);
                }
            }
        }
    }
}
