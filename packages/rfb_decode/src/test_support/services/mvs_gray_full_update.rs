use super::{mvs_bits, mvs_full_update, mvs_marker};

pub fn mvs_gray_full_update() -> Vec<u8> {
    mvs_full_update(
        15,
        25,
        &mvs_bits(&["0", "101 1 0000", &mvs_marker("m")]),
        &mvs_bits(&[
            "0 0 0",
            "00",
            "00",
            "1110 010 1",
            "0 01 0",
            "1",
            &mvs_marker("m"),
        ]),
    )
}
