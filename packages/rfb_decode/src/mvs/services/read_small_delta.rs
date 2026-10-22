use super::step_away_from_zero;
use crate::{DecodeError, MvsBits};

pub(crate) fn read_small_delta(bits: &mut MvsBits<'_>, baseline: i8) -> Result<i8, DecodeError> {
    let bit = bits.read_bit()?;
    if baseline != 0 {
        return Ok(step_away_from_zero(baseline, bit));
    }
    if bit == 0 {
        return Ok(0);
    }
    Ok(if bits.read_bit()? == 1 { -1 } else { 1 })
}
