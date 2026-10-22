use super::Rect;

impl Rect {
    pub fn area(&self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }
}
