use rfb::Rect;

use super::Framebuffer;

impl Framebuffer {
    pub fn copy_rect(&mut self, src_x: u16, src_y: u16, dst: Rect) {
        let src = Rect::new(src_x, src_y, dst.width, dst.height);
        let in_bounds =
            dst.fits_within(self.width, self.height) && src.fits_within(self.width, self.height);
        if dst.is_empty() || !in_bounds {
            return;
        }
        let stride = usize::from(self.width);
        let row_len = usize::from(dst.width);
        let bottom_up = dst.y > src_y;
        for step in 0..dst.height {
            let row = if bottom_up {
                dst.height - 1 - step
            } else {
                step
            };
            let from = (usize::from(src_y) + usize::from(row)) * stride + usize::from(src_x);
            let to = (usize::from(dst.y) + usize::from(row)) * stride + usize::from(dst.x);
            self.pixels.copy_within(from..from + row_len, to);
        }
    }
}
