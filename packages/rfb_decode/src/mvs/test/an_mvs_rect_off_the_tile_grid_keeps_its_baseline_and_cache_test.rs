use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_cache_slot_one_partial_update, mvs_gray_full_update,
    mvs_red_step_partial_update, mvs_unchanged_partial_update, rgba_pixel,
};

#[test]
fn test_an_mvs_rect_off_the_tile_grid_keeps_its_baseline_and_cache() {
    let mut harness = DecodeHarness::new(32, 16);
    let rect = Rect::new(16, 3, 16, 8);

    harness.decode_mvs(rect, &mvs_gray_full_update());
    harness.decode_mvs(rect, &mvs_red_step_partial_update());
    harness.decode_mvs(rect, &mvs_unchanged_partial_update());
    harness.decode_mvs(rect, &mvs_cache_slot_one_partial_update());

    assert_eq!(
        harness.pixel(16, 3),
        rgba_pixel(148, 148, 148),
        "the first tile is back on its full update baseline"
    );
    assert_eq!(
        harness.pixel(24, 3),
        rgba_pixel(151, 147, 148),
        "the second tile comes from cache slot 1"
    );
}
