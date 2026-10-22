use std::num::Wrapping;

use crate::{
    MVS_FIX_0_298631336, MVS_FIX_0_390180644, MVS_FIX_0_541196100, MVS_FIX_0_765366865,
    MVS_FIX_0_899976223, MVS_FIX_1_175875602, MVS_FIX_1_501321110, MVS_FIX_1_847759065,
    MVS_FIX_1_961570560, MVS_FIX_2_053119869, MVS_FIX_2_562915447, MVS_FIX_3_072711026,
    MVS_IDCT_CONST_BITS,
};

pub(crate) fn idct_butterfly(input: [Wrapping<i32>; 8]) -> [Wrapping<i32>; 8] {
    let even = (input[2] + input[6]) * Wrapping(MVS_FIX_0_541196100);
    let even2 = even - input[6] * Wrapping(MVS_FIX_1_847759065);
    let even3 = even + input[2] * Wrapping(MVS_FIX_0_765366865);
    let even0 = (input[0] + input[4]) << MVS_IDCT_CONST_BITS;
    let even1 = (input[0] - input[4]) << MVS_IDCT_CONST_BITS;
    let sum10 = even0 + even3;
    let sum13 = even0 - even3;
    let sum11 = even1 + even2;
    let sum12 = even1 - even2;

    let z3 = input[7] + input[3];
    let z4 = input[5] + input[1];
    let z5 = (z3 + z4) * Wrapping(MVS_FIX_1_175875602);
    let z1 = (input[7] + input[1]) * Wrapping(-MVS_FIX_0_899976223);
    let z2 = (input[5] + input[3]) * Wrapping(-MVS_FIX_2_562915447);
    let z3 = z3 * Wrapping(-MVS_FIX_1_961570560) + z5;
    let z4 = z4 * Wrapping(-MVS_FIX_0_390180644) + z5;
    let odd0 = input[7] * Wrapping(MVS_FIX_0_298631336) + z1 + z3;
    let odd1 = input[5] * Wrapping(MVS_FIX_2_053119869) + z2 + z4;
    let odd2 = input[3] * Wrapping(MVS_FIX_3_072711026) + z2 + z3;
    let odd3 = input[1] * Wrapping(MVS_FIX_1_501321110) + z1 + z4;

    [
        sum10 + odd3,
        sum11 + odd2,
        sum12 + odd1,
        sum13 + odd0,
        sum13 - odd0,
        sum12 - odd1,
        sum11 - odd2,
        sum10 - odd3,
    ]
}
