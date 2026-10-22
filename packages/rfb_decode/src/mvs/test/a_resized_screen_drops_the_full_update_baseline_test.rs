use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_gray_full_update, mvs_red_step_partial_update, rgba_pixel,
};

#[test]
fn test_a_resized_screen_drops_the_full_update_baseline() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);
    harness.decode_mvs(rect, &mvs_gray_full_update());

    harness.framebuffer.resize(24, 8);
    harness.decode_mvs(rect, &mvs_red_step_partial_update());

    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(148, 148, 148),
        "a partial update has nothing to build on until a full update arrives for the new size"
    );
}
