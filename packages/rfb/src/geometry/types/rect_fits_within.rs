use super::Rect;

impl Rect {
    pub fn fits_within(&self, width: u16, height: u16) -> bool {
        u32::from(self.x) + u32::from(self.width) <= u32::from(width)
            && u32::from(self.y) + u32::from(self.height) <= u32::from(height)
    }
}
