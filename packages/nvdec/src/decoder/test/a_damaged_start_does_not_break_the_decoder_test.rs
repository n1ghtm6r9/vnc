use crate::test_support::{
    FOUR_COLOURS_444, access_units, assert_four_colour_frames, decode_stream, test_decoder,
};

#[test]
fn test_a_damaged_start_does_not_break_the_decoder() {
    let Some(mut decoder) = test_decoder() else {
        return;
    };
    let units = access_units(FOUR_COLOURS_444);

    let _ = decoder.decode(units[1]);
    let _ = decoder.decode(&units[0][..units[0].len() / 2]);
    let frames = decode_stream(&mut decoder, FOUR_COLOURS_444);

    assert_four_colour_frames(&frames);
}
