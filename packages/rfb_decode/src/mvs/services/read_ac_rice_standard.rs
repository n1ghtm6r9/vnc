use super::read_signed;
use crate::{DecodeError, MvsBits};

pub(crate) fn read_ac_rice_standard(bits: &mut MvsBits<'_>) -> Result<i32, DecodeError> {
    let ones = bits.leading_ones()?;
    let magnitude = match ones {
        0 => 2 + bits.read_bits(1)? as i32,
        1 => 4 + bits.read_bits(2)? as i32,
        _ => ((ones as i32).wrapping_shl(3) | bits.read_bits(3)? as i32).wrapping_sub(8),
    };
    read_signed(bits, magnitude)
}
