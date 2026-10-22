use super::PixelConverter;

impl PixelConverter {
    #[inline]
    pub(super) fn raw_value(&self, bytes: &[u8]) -> u32 {
        let used = bytes.iter().take(self.bytes_per_pixel).copied();
        if self.big_endian {
            used.fold(0, |value, byte| (value << 8) | u32::from(byte))
        } else {
            used.enumerate().fold(0, |value, (index, byte)| {
                value | (u32::from(byte) << (8 * index))
            })
        }
    }
}
