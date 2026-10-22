use rfb::Rect;

use crate::test_support::{DecodeHarness, mvs_gray_full_update, rgba_pixel};

#[test]
fn test_an_mvs_rect_need_not_start_on_a_tile_boundary() {
    let mut harness = DecodeHarness::new(32, 16);

    harness.decode_mvs(Rect::new(16, 3, 16, 8), &mvs_gray_full_update());

    let gray = rgba_pixel(148, 148, 148);
    let black = rgba_pixel(0, 0, 0);
    assert_eq!(harness.pixel(16, 3), gray, "the first pixel of the rect");
    assert_eq!(harness.pixel(31, 10), gray, "the last pixel of the rect");
    assert_eq!(harness.pixel(16, 2), black, "the row above the rect");
    assert_eq!(harness.pixel(16, 11), black, "the row below the rect");
    assert_eq!(harness.pixel(15, 3), black, "the column left of the rect");
}
