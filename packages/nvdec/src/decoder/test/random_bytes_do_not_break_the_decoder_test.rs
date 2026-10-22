use crate::test_support::{
    FOUR_COLOURS_444, assert_four_colour_frames, decode_stream, pseudo_random_bytes, test_decoder,
};

#[test]
fn test_random_bytes_do_not_break_the_decoder() {
    let Some(mut decoder) = test_decoder() else {
        return;
    };
    let kinds = [32, 33, 34, 19, 1];

    for seed in 1..=8 {
        let mut garbage = pseudo_random_bytes(4096, seed);
        for (index, unit) in garbage.chunks_mut(512).enumerate() {
            unit[..5].copy_from_slice(&[0, 0, 1, kinds[index % kinds.len()] << 1, 1]);
        }
        let _ = decoder.decode(&garbage);
    }
    let frames = decode_stream(&mut decoder, FOUR_COLOURS_444);

    assert_four_colour_frames(&frames);
}
