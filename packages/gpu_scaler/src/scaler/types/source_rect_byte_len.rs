use super::SourceRect;

impl SourceRect {
    pub(crate) fn byte_len(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height) * 4
    }
}
