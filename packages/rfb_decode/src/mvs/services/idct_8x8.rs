use std::num::Wrapping;

use super::{idct_butterfly, range_limit};
use crate::{MVS_IDCT_CONST_BITS, MVS_IDCT_PASS1_BITS};

pub(crate) fn idct_8x8(block: &[i32; 64], output: &mut [u8; 64]) {
    let mut workspace = [Wrapping(0i32); 64];
    let column_shift = MVS_IDCT_CONST_BITS - MVS_IDCT_PASS1_BITS;
    let column_round = Wrapping(1 << (column_shift - 1));
    for column in 0..8 {
        let input: [Wrapping<i32>; 8] =
            std::array::from_fn(|row| Wrapping(block[row * 8 + column]));
        let lanes = if input[1..].iter().all(|value| value.0 == 0) {
            [input[0] << MVS_IDCT_PASS1_BITS; 8]
        } else {
            idct_butterfly(input).map(|value| (value + column_round) >> column_shift)
        };
        for (row, value) in lanes.into_iter().enumerate() {
            workspace[row * 8 + column] = value;
        }
    }
    let row_shift = MVS_IDCT_CONST_BITS + MVS_IDCT_PASS1_BITS + 3;
    let row_round = Wrapping(1 << (row_shift - 1));
    let (rows, _) = output.as_chunks_mut::<8>();
    let (lanes, _) = workspace.as_chunks::<8>();
    for (samples, &input) in rows.iter_mut().zip(lanes) {
        for (sample, value) in samples.iter_mut().zip(idct_butterfly(input)) {
            *sample = range_limit(((value + row_round) >> row_shift).0);
        }
    }
}
