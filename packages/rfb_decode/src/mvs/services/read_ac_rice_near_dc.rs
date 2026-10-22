use super::read_signed;
use crate::{DecodeError, MvsBits};

pub(crate) fn read_ac_rice_near_dc(bits: &mut MvsBits<'_>) -> Result<i32, DecodeError> {
    let ones = bits.leading_ones()?;
    let magnitude = if ones <= 3 {
        2 + ones as i32 * 4 + bits.read_bits(2)? as i32
    } else {
        (ones as i32)
            .wrapping_sub(4)
            .wrapping_mul(8)
            .wrapping_add(18)
            .wrapping_add(bits.read_bits(3)? as i32)
    };
    read_signed(bits, magnitude)
}
