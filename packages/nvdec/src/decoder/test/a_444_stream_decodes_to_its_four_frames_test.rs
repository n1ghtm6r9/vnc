use crate::test_support::{
    FOUR_COLOURS_444, assert_four_colour_frames, decode_stream, test_decoder,
};

#[test]
fn test_a_444_stream_decodes_to_its_four_frames() {
    let Some(mut decoder) = test_decoder() else {
        return;
    };

    let frames = decode_stream(&mut decoder, FOUR_COLOURS_444);

    assert_four_colour_frames(&frames);
}
