use super::{CompactLayout, PixelConverter};

impl PixelConverter {
    #[inline]
    pub(super) fn compact_value(&self, bytes: &[u8]) -> u32 {
        if self.compact == CompactLayout::Full {
            return self.raw_value(bytes);
        }
        let [first, second, third] = [0, 1, 2].map(|index| bytes.get(index).copied().unwrap_or(0));
        match (self.compact, self.big_endian) {
            (CompactLayout::MostSignificantThree, false) => {
                u32::from_le_bytes([0, first, second, third])
            }
            (CompactLayout::MostSignificantThree, true) => {
                u32::from_be_bytes([first, second, third, 0])
            }
            (_, false) => u32::from_le_bytes([first, second, third, 0]),
            (_, true) => u32::from_be_bytes([0, first, second, third]),
        }
    }
}
