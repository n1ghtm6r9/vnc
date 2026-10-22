use rfb::{ENCODING_HEXTILE, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel, wire_pixel};

#[test]
fn test_hextile_foreground_applies_to_uncoloured_subrects() {
    let mut harness = DecodeHarness::new(32, 16);
    let mut payload = vec![2 | 4 | 8];
    payload.extend_from_slice(&wire_pixel(1, 1, 1));
    payload.extend_from_slice(&wire_pixel(9, 9, 9));
    payload.push(2);
    payload.extend_from_slice(&[0x00, 0x11, 0x33, 0x00]);
    payload.extend_from_slice(&[8, 1, 0x55, 0x00]);
    harness
        .decode(ENCODING_HEXTILE, Rect::new(0, 0, 32, 16), &payload)
        .expect("a hextile rect with a foreground decodes");
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(9, 9, 9),
        "the first subrect uses the foreground"
    );
    assert_eq!(
        harness.pixel(1, 1),
        rgba_pixel(9, 9, 9),
        "the first subrect is two by two"
    );
    assert_eq!(
        harness.pixel(2, 2),
        rgba_pixel(1, 1, 1),
        "the pixel past the first subrect shows the background"
    );
    assert_eq!(
        harness.pixel(3, 3),
        rgba_pixel(9, 9, 9),
        "the second subrect uses the foreground"
    );
    assert_eq!(
        harness.pixel(16, 0),
        rgba_pixel(1, 1, 1),
        "the second tile reuses the background"
    );
    assert_eq!(
        harness.pixel(21, 5),
        rgba_pixel(9, 9, 9),
        "the second tile reuses the foreground"
    );
}
