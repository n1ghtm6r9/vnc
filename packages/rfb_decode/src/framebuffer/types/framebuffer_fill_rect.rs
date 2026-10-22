use rfb::Rect;

use super::Framebuffer;

impl Framebuffer {
    pub fn fill_rect(&mut self, rect: Rect, rgba: u32) {
        if rect.is_empty() || !rect.fits_within(self.width, self.height) {
            return;
        }
        for y in rect.y..rect.y + rect.height {
            self.row_mut(rect.x, y, rect.width).fill(rgba);
        }
    }
}
