use super::{idct_8x8, idct_flat};
use crate::MVS_ZIGZAG;

pub(crate) fn transform_plane(
    plane: &[i16; 64],
    quantization: &[u8; 64],
    output: &mut [u8; 64],
) -> Option<u8> {
    let mut block = [0i32; 64];
    let mut ac = 0;
    for (position, &value) in plane.iter().enumerate().skip(1) {
        let natural = MVS_ZIGZAG[position];
        ac |= value;
        block[natural] = i32::from(value) * i32::from(quantization[natural]);
    }
    let dc = i32::from(plane[0]) * i32::from(quantization[0]);
    if ac == 0 {
        return Some(idct_flat(dc));
    }
    block[0] = dc;
    idct_8x8(&block, output);
    None
}
