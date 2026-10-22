pub(crate) fn channel_bits(max: u16, shift: u8) -> Option<u32> {
    let shift = u32::from(shift);
    if shift >= u32::BITS {
        return None;
    }
    u32::try_from(u64::from(max) << shift).ok()
}
