use super::Framebuffer;
use crate::OPAQUE_BLACK;

impl Framebuffer {
    pub fn resize(&mut self, width: u16, height: u16) {
        if width == self.width && height == self.height {
            return;
        }
        let mut pixels = vec![OPAQUE_BLACK; usize::from(width) * usize::from(height)];
        let kept_width = usize::from(self.width.min(width));
        let kept_height = usize::from(self.height.min(height));
        let old_stride = usize::from(self.width);
        let new_stride = usize::from(width);
        for row in 0..kept_height {
            let from = row * old_stride;
            let to = row * new_stride;
            pixels[to..to + kept_width].copy_from_slice(&self.pixels[from..from + kept_width]);
        }
        self.width = width;
        self.height = height;
        self.pixels = pixels;
    }
}
