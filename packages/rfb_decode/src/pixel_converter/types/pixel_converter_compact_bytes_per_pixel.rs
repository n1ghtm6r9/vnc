use super::{CompactLayout, PixelConverter};

impl PixelConverter {
    pub fn compact_bytes_per_pixel(&self) -> usize {
        match self.compact {
            CompactLayout::Full => self.bytes_per_pixel,
            CompactLayout::LeastSignificantThree | CompactLayout::MostSignificantThree => 3,
        }
    }
}
