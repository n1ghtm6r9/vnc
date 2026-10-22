use super::PixelConverter;

impl PixelConverter {
    #[inline]
    pub fn compact_rgba(&self, bytes: &[u8]) -> u32 {
        self.rgba_from_value(self.compact_value(bytes))
    }
}
