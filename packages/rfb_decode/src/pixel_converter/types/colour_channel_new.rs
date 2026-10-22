use super::ColourChannel;

impl ColourChannel {
    pub(crate) fn new(max: u16, shift: u8) -> ColourChannel {
        let divisor = u32::from(max.max(1));
        let table = (0..=u32::from(max))
            .map(|value| u8::try_from((value * 255 + divisor / 2) / divisor).unwrap_or(u8::MAX))
            .collect();
        ColourChannel {
            shift: u32::from(shift).min(31),
            mask: u32::from(max),
            table,
        }
    }
}
