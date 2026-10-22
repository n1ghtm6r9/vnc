use rfb::Rect;

use super::Framebuffer;

impl Framebuffer {
    pub fn read_rect(&self, rect: Rect) -> Vec<u32> {
        if rect.is_empty() || !rect.fits_within(self.width, self.height) {
            return Vec::new();
        }
        let stride = usize::from(self.width);
        let row_len = usize::from(rect.width);
        let mut pixels = Vec::with_capacity(rect.area());
        for y in rect.y..rect.y + rect.height {
            let start = usize::from(y) * stride + usize::from(rect.x);
            pixels.extend_from_slice(&self.pixels[start..start + row_len]);
        }
        pixels
    }
}
