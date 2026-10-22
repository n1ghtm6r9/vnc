use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_cache_slot_one_partial_update, mvs_gray_full_update,
    mvs_red_step_partial_update, mvs_unchanged_partial_update, rgba_pixel,
};

#[test]
fn test_partial_dct_tiles_fill_the_cache_from_slot_one() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);

    harness.decode_mvs(rect, &mvs_gray_full_update());
    harness.decode_mvs(rect, &mvs_red_step_partial_update());
    harness.decode_mvs(rect, &mvs_unchanged_partial_update());
    harness.decode_mvs(rect, &mvs_cache_slot_one_partial_update());

    assert_eq!(
        harness.pixel(8, 0),
        rgba_pixel(151, 147, 148),
        "slot 1 holds the first partial DCT tile, the red one"
    );
}
