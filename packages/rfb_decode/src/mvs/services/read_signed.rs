use crate::{DecodeError, MvsBits};

pub(crate) fn read_signed(bits: &mut MvsBits<'_>, magnitude: i32) -> Result<i32, DecodeError> {
    if bits.read_bit()? == 1 {
        Ok(magnitude.wrapping_neg())
    } else {
        Ok(magnitude)
    }
}
