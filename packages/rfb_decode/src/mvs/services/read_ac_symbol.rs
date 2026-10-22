use super::{quantization_step, read_ac_rice_near_dc, read_ac_rice_standard, read_skip_run};
use crate::{DecodeError, MvsAcSymbol, MvsBits};

pub(crate) fn read_ac_symbol(
    bits: &mut MvsBits<'_>,
    position: usize,
    threshold: u8,
) -> Result<MvsAcSymbol, DecodeError> {
    let step = quantization_step(position, threshold);
    if bits.read_bit()? == 1 {
        let raw = if position <= 5 {
            read_ac_rice_near_dc(bits)?
        } else {
            read_ac_rice_standard(bits)?
        };
        return Ok(MvsAcSymbol::Value(raw.wrapping_mul(step)));
    }
    Ok(match bits.read_bits(2)? {
        0 => MvsAcSymbol::Value(0),
        1 => {
            if bits.read_bit()? == 0 {
                MvsAcSymbol::EndOfBlock
            } else {
                MvsAcSymbol::Skip(read_skip_run(bits, position)?)
            }
        }
        2 => MvsAcSymbol::Value(step),
        _ => MvsAcSymbol::Value(-step),
    })
}
