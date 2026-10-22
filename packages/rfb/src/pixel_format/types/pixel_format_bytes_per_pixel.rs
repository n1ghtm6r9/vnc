use super::PixelFormat;

impl PixelFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        usize::from(self.bits_per_pixel) / 8
    }
}
