use super::PixelConverter;

impl PixelConverter {
    pub(crate) fn tight_is_rgb24(&self) -> bool {
        self.tight_rgb24
    }
}
