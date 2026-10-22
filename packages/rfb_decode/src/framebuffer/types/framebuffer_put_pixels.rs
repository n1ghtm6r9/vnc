use rfb::Rect;

use super::Framebuffer;

impl Framebuffer {
    pub fn put_pixels(&mut self, rect: Rect, pixels: &[u32]) {
        if rect.is_empty()
            || !rect.fits_within(self.width, self.height)
            || pixels.len() < rect.area()
        {
            return;
        }
        let rows = pixels.chunks_exact(usize::from(rect.width));
        for (y, source) in (rect.y..rect.y + rect.height).zip(rows) {
            self.row_mut(rect.x, y, rect.width).copy_from_slice(source);
        }
    }
}
