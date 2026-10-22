use crate::{DecodeError, MvsBits};

pub(crate) fn read_two_colour_bitmap(
    bits: &mut MvsBits<'_>,
    set: u32,
    clear: u32,
) -> Result<[u32; 64], DecodeError> {
    let full_rows = bits.read_bits(8)?;
    let mut pixels = [clear; 64];
    for (row, line) in pixels.as_chunks_mut::<8>().0.iter_mut().enumerate() {
        let row_bits = if (full_rows >> (7 - row)) & 1 == 1 {
            0xFF
        } else {
            bits.read_bits(8)?
        };
        for (column, pixel) in line.iter_mut().enumerate() {
            if (row_bits >> (7 - column)) & 1 == 1 {
                *pixel = set;
            }
        }
    }
    Ok(pixels)
}
