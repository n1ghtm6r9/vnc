use super::Rect;

impl Rect {
    pub fn union(&self, other: Rect) -> Rect {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return *self;
        }
        let left = self.x.min(other.x);
        let top = self.y.min(other.y);
        let right = (u32::from(self.x) + u32::from(self.width))
            .max(u32::from(other.x) + u32::from(other.width));
        let bottom = (u32::from(self.y) + u32::from(self.height))
            .max(u32::from(other.y) + u32::from(other.height));
        let width = u16::try_from(right - u32::from(left)).unwrap_or(u16::MAX);
        let height = u16::try_from(bottom - u32::from(top)).unwrap_or(u16::MAX);
        Rect::new(left, top, width, height)
    }
}
