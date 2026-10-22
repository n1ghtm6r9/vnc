use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_gray_full_update, mvs_red_step_partial_update, mvs_unchanged_partial_update,
    rgba_pixel,
};

#[test]
fn test_every_partial_update_is_coded_against_the_last_full_update() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);

    harness.decode_mvs(rect, &mvs_gray_full_update());
    harness.decode_mvs(rect, &mvs_red_step_partial_update());
    harness.decode_mvs(rect, &mvs_unchanged_partial_update());

    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(148, 148, 148),
        "unchanged means unchanged from the full update, not from the partial update before"
    );
}
