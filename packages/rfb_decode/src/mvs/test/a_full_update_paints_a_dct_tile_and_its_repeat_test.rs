use rfb::Rect;

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, mvs_gray_full_update, rgba_pixel};

#[test]
fn test_a_full_update_paints_a_dct_tile_and_its_repeat() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);

    let outcome = harness.decode_mvs(rect, &mvs_gray_full_update());

    assert_eq!(
        outcome,
        RectOutcome::Pixels(rect),
        "a full update repaints its whole rect"
    );
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(148, 148, 148),
        "the DCT tile is the gray its luma DC of 10 gives with the default tables"
    );
    assert_eq!(
        harness.pixel(15, 7),
        rgba_pixel(148, 148, 148),
        "the repeat tile is a copy of the DCT tile before it"
    );
}
