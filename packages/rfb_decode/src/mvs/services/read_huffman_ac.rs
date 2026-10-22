use super::read_huffman_symbol;
use crate::{DecodeError, MvsBits};

pub(crate) fn read_huffman_ac(
    bits: &mut MvsBits<'_>,
    plane: &mut [i16; 64],
    limit: usize,
) -> Result<(), DecodeError> {
    let mut consumed = 0;
    while consumed < limit {
        let symbol = read_huffman_symbol(bits)?;
        let run = usize::from(symbol >> 4);
        let size = u32::from(symbol & 0x0F);
        if size == 0 {
            if run != 15 {
                return Ok(());
            }
            consumed += 16;
            continue;
        }
        consumed += run;
        let raw = bits.read_bits(size)? as i32;
        let value = if raw < 1 << (size - 1) {
            raw - (1 << size) + 1
        } else {
            raw
        };
        plane[(consumed + 1).min(63)] = value as i16;
        consumed += 1;
    }
    Ok(())
}
