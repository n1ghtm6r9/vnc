use super::SourceRect;

impl SourceRect {
    pub(crate) const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}
