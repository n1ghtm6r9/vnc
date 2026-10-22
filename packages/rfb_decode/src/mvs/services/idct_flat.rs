use std::num::Wrapping;

use super::range_limit;
use crate::MVS_IDCT_PASS1_BITS;

pub(crate) fn idct_flat(dc: i32) -> u8 {
    let scaled = (Wrapping(dc) << MVS_IDCT_PASS1_BITS) + Wrapping(1 << (MVS_IDCT_PASS1_BITS + 2));
    range_limit((scaled >> (MVS_IDCT_PASS1_BITS + 3)).0)
}
