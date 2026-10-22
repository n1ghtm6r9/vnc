use super::{mvs_bits, mvs_marker, mvs_partial_update};

pub fn mvs_cache_slot_one_partial_update() -> Vec<u8> {
    mvs_partial_update(
        14,
        19,
        &mvs_bits(&["00", "11 0 0000000000000001", &mvs_marker("mvs")]),
    )
}
