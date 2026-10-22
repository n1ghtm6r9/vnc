use crate::test_support::{
    FOUR_COLOURS_420, FOUR_COLOURS_444, assert_four_colour_frames, decode_stream, test_decoder,
};

#[test]
fn test_a_change_of_stream_format_rebuilds_the_decoder() {
    let Some(mut decoder) = test_decoder() else {
        return;
    };

    let first = decode_stream(&mut decoder, FOUR_COLOURS_444);
    let second = decode_stream(&mut decoder, FOUR_COLOURS_420);

    assert_four_colour_frames(&first);
    assert_four_colour_frames(&second);
}
