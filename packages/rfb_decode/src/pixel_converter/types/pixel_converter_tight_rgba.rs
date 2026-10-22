use super::PixelConverter;
use crate::OPAQUE_BLACK;

impl PixelConverter {
    #[inline]
    pub fn tight_rgba(&self, bytes: &[u8]) -> u32 {
        if !self.tight_rgb24 {
            return self.rgba(bytes);
        }
        match bytes {
            [red, green, blue, ..] => u32::from_le_bytes([*red, *green, *blue, 0xFF]),
            _ => OPAQUE_BLACK,
        }
    }
}
