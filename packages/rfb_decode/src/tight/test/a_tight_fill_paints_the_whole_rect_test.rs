use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, compact_pixel, rgba_pixel};

#[test]
fn test_a_tight_fill_paints_the_whole_rect() {
    let mut harness = DecodeHarness::new(8, 6);
    let mut payload = vec![0x80u8];
    payload.extend_from_slice(&compact_pixel(12, 34, 56));
    harness
        .decode(ENCODING_TIGHT, Rect::new(2, 1, 4, 3), &payload)
        .expect("a fill rect decodes");
    for y in 1..4 {
        for x in 2..6 {
            assert_eq!(
                harness.pixel(x, y),
                rgba_pixel(12, 34, 56),
                "filled pixel ({x}, {y}) has the fill colour"
            );
        }
    }
    assert_eq!(
        harness.pixel(1, 1),
        rgba_pixel(0, 0, 0),
        "the pixel left of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(6, 1),
        rgba_pixel(0, 0, 0),
        "the pixel right of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(2, 4),
        rgba_pixel(0, 0, 0),
        "the pixel below the rect is untouched"
    );
}
