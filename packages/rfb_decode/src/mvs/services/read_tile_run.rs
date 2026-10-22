use crate::{DecodeError, MvsBits};

pub(crate) fn read_tile_run(bits: &mut MvsBits<'_>) -> Result<usize, DecodeError> {
    if bits.read_bit()? == 0 {
        return Ok(1);
    }
    let short = bits.read_bits(4)?;
    if short < 15 {
        return Ok(short as usize + 2);
    }
    let first = bits.read_bits(8)?;
    let long = if first & 0x80 == 0 {
        first
    } else {
        let second = bits.read_bits(8)?;
        if second & 0x80 == 0 {
            (second << 7) | (first & 0x7F)
        } else {
            let third = bits.read_bits(8)?;
            (third << 14) | ((second & 0x7F) << 7) | (first & 0x7F)
        }
    };
    Ok(long as usize + 17)
}
