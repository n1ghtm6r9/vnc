use super::{mvs_bits, mvs_marker, mvs_partial_update};
use crate::test_support::MVS_HUFFMAN_END_OF_BLOCK;

pub fn mvs_red_step_partial_update() -> Vec<u8> {
    mvs_partial_update(
        14,
        19,
        &mvs_bits(&[
            "01",
            "000000",
            "1 0",
            MVS_HUFFMAN_END_OF_BLOCK,
            "0",
            MVS_HUFFMAN_END_OF_BLOCK,
            "00",
            &mvs_marker("mvs"),
        ]),
    )
}
