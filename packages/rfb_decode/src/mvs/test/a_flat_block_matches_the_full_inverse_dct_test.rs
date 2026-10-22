use crate::{idct_8x8, idct_flat};

#[test]
fn test_a_flat_block_matches_the_full_inverse_dct() {
    let mut block = [0i32; 64];
    let mut samples = [0u8; 64];
    for dc in -40_000..=40_000 {
        block[0] = dc;
        idct_8x8(&block, &mut samples);
        let flat = idct_flat(dc);
        assert!(
            samples.iter().all(|&sample| sample == flat),
            "a block with only a DC of {dc} is flat at {flat}, the full transform gives {samples:?}"
        );
    }
}
