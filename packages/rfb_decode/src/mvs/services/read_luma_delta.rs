use super::{read_dc_rice, step_away_from_zero};
use crate::{DecodeError, MvsBits};

pub(crate) fn read_luma_delta(
    bits: &mut MvsBits<'_>,
    baseline: i8,
    delta_bits: u32,
) -> Result<i8, DecodeError> {
    if baseline == 0 {
        return Ok(read_dc_rice(bits)? as i8);
    }
    Ok(step_away_from_zero(baseline, bits.read_bits(delta_bits)?))
}
