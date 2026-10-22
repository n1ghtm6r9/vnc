use super::{chroma_offsets, luma_pixel};
use crate::{DecodeError, MvsBits};

pub(crate) fn read_ycbcr20(bits: &mut MvsBits<'_>) -> Result<u32, DecodeError> {
    let luma = bits.read_bits(8)? as u8;
    let blue = (bits.read_bits(6)? << 2) as u8;
    let red = (bits.read_bits(6)? << 2) as u8;
    Ok(luma_pixel(luma, chroma_offsets(blue, red)))
}
