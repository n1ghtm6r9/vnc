use super::PixelConverter;

impl PixelConverter {
    pub fn tight_bytes_per_pixel(&self) -> usize {
        if self.tight_rgb24 {
            3
        } else {
            self.bytes_per_pixel
        }
    }
}
