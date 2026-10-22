use super::ColourChannel;

impl ColourChannel {
    #[inline]
    pub(crate) fn scale(&self, pixel_value: u32) -> u8 {
        let index = (pixel_value >> self.shift) & self.mask;
        self.table.get(index as usize).copied().unwrap_or(0)
    }
}
