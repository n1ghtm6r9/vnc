use super::SourceRect;

impl SourceRect {
    pub(crate) fn fits_within(&self, width: u32, height: u32) -> bool {
        u64::from(self.x) + u64::from(self.width) <= u64::from(width)
            && u64::from(self.y) + u64::from(self.height) <= u64::from(height)
    }
}
