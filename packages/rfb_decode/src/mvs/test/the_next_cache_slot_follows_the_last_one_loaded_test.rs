use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_cache_next_partial_update, mvs_cache_slot_one_partial_update,
    mvs_gray_full_update, mvs_red_step_partial_update, mvs_unchanged_partial_update, rgba_pixel,
};

#[test]
fn test_the_next_cache_slot_follows_the_last_one_loaded() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);

    harness.decode_mvs(rect, &mvs_gray_full_update());
    harness.decode_mvs(rect, &mvs_red_step_partial_update());
    harness.decode_mvs(rect, &mvs_unchanged_partial_update());
    harness.decode_mvs(rect, &mvs_cache_slot_one_partial_update());
    harness.decode_mvs(rect, &mvs_cache_next_partial_update());

    assert_eq!(
        harness.pixel(8, 0),
        rgba_pixel(148, 148, 148),
        "the slot after slot 1 holds the second partial DCT tile, the gray one"
    );
}
