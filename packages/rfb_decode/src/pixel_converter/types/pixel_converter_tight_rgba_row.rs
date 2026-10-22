use super::PixelConverter;
use crate::rgb24_row_to_rgba;

impl PixelConverter {
    pub(crate) fn tight_rgba_row(&self, src: &[u8], dst: &mut [u32]) {
        if self.tight_rgb24 {
            rgb24_row_to_rgba(src, dst);
        } else {
            self.rgba_row(src, dst);
        }
    }
}
