use super::PixelConverter;
use crate::OPAQUE_BLACK;

impl PixelConverter {
    #[inline]
    pub fn rgba(&self, bytes: &[u8]) -> u32 {
        if self.rgba32_layout {
            return match bytes {
                [red, green, blue, ..] => u32::from_le_bytes([*red, *green, *blue, 0xFF]),
                _ => OPAQUE_BLACK,
            };
        }
        self.rgba_from_value(self.raw_value(bytes))
    }
}
