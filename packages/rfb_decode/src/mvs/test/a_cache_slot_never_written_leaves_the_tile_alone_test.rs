use rfb::Rect;

use crate::test_support::{
    DecodeHarness, mvs_cache_slot_one_partial_update, mvs_gray_full_update, rgba_pixel,
};

#[test]
fn test_a_cache_slot_never_written_leaves_the_tile_alone() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);

    harness.decode_mvs(rect, &mvs_gray_full_update());
    harness.decode_mvs(rect, &mvs_cache_slot_one_partial_update());

    assert_eq!(
        harness.pixel(8, 0),
        rgba_pixel(148, 148, 148),
        "an empty cache slot paints nothing over the tile"
    );
}
