use super::PixelConverter;
use crate::OPAQUE_BLACK;

impl PixelConverter {
    #[inline]
    pub(super) fn rgba_from_value(&self, value: u32) -> u32 {
        if !self.true_colour {
            return self
                .colour_map
                .get(value as usize)
                .copied()
                .unwrap_or(OPAQUE_BLACK);
        }
        u32::from_le_bytes([
            self.red.scale(value),
            self.green.scale(value),
            self.blue.scale(value),
            0xFF,
        ])
    }
}
