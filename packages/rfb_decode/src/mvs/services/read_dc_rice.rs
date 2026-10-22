use super::read_signed;
use crate::{DecodeError, MvsBits};

pub(crate) fn read_dc_rice(bits: &mut MvsBits<'_>) -> Result<i32, DecodeError> {
    let ones = bits.leading_ones()?;
    let magnitude = match ones {
        0 => {
            if bits.read_bit()? == 0 {
                return Ok(0);
            }
            1
        }
        1 => 2 + bits.read_bits(1)? as i32,
        2 => 4 + bits.read_bits(2)? as i32,
        _ => ((ones as i32).wrapping_shl(3) | bits.read_bits(3)? as i32).wrapping_sub(16),
    };
    read_signed(bits, magnitude)
}
