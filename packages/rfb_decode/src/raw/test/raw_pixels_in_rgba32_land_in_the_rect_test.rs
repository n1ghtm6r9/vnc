use rfb::{ENCODING_RAW, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel, wire_pixel};

#[test]
fn test_raw_pixels_in_rgba32_land_in_the_rect() {
    let mut harness = DecodeHarness::new(8, 6);
    let rect = Rect::new(2, 1, 3, 2);
    let payload = [
        wire_pixel(1, 2, 3),
        wire_pixel(4, 5, 6),
        wire_pixel(7, 8, 9),
        wire_pixel(10, 11, 12),
        wire_pixel(13, 14, 15),
        wire_pixel(16, 17, 18),
    ]
    .concat();
    let outcome = harness
        .decode(ENCODING_RAW, rect, &payload)
        .expect("a well formed raw rect decodes");
    assert_eq!(
        outcome,
        RectOutcome::Pixels(rect),
        "the outcome names the painted rect"
    );
    assert_eq!(
        harness.pixel(2, 1),
        rgba_pixel(1, 2, 3),
        "the first pixel lands in the top left corner"
    );
    assert_eq!(
        harness.pixel(4, 1),
        rgba_pixel(7, 8, 9),
        "the third pixel ends the first row"
    );
    assert_eq!(
        harness.pixel(2, 2),
        rgba_pixel(10, 11, 12),
        "the fourth pixel starts the second row"
    );
    assert_eq!(
        harness.pixel(4, 2),
        rgba_pixel(16, 17, 18),
        "the last pixel lands in the bottom right corner"
    );
    assert_eq!(
        harness.pixel(1, 1),
        rgba_pixel(0, 0, 0),
        "the pixel left of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(5, 1),
        rgba_pixel(0, 0, 0),
        "the pixel right of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(2, 3),
        rgba_pixel(0, 0, 0),
        "the pixel below the rect is untouched"
    );
}
