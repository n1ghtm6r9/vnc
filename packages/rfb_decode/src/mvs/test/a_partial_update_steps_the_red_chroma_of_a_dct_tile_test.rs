use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_gray_full_update, mvs_red_step_partial_update, rgba_pixel,
};

#[test]
fn test_a_partial_update_steps_the_red_chroma_of_a_dct_tile() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);

    harness.decode_mvs(rect, &mvs_gray_full_update());
    harness.decode_mvs(rect, &mvs_red_step_partial_update());

    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(151, 147, 148),
        "one step of red chroma tints the recoded tile"
    );
    assert_eq!(
        harness.pixel(8, 0),
        rgba_pixel(148, 148, 148),
        "the skipped tile keeps its gray"
    );
}
