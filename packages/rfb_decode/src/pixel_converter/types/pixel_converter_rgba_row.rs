use super::PixelConverter;
use crate::rgbx32_row_to_rgba;

impl PixelConverter {
    pub(crate) fn rgba_row(&self, src: &[u8], dst: &mut [u32]) {
        if self.rgba32_layout {
            rgbx32_row_to_rgba(src, dst);
            return;
        }
        for (pixel, bytes) in dst.iter_mut().zip(src.chunks_exact(self.bytes_per_pixel)) {
            *pixel = self.rgba(bytes);
        }
    }
}
