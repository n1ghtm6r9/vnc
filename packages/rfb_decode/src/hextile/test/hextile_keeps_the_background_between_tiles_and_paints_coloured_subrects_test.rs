use rfb::{ENCODING_HEXTILE, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel, wire_pixel};

#[test]
fn test_hextile_keeps_the_background_between_tiles_and_paints_coloured_subrects() {
    let mut harness = DecodeHarness::new(32, 16);
    let mut payload = vec![2 | 8 | 16];
    payload.extend_from_slice(&wire_pixel(10, 20, 30));
    payload.push(1);
    payload.extend_from_slice(&wire_pixel(200, 0, 0));
    payload.extend_from_slice(&[0x32, 0x31]);
    payload.push(8 | 16);
    payload.push(1);
    payload.extend_from_slice(&wire_pixel(0, 200, 0));
    payload.extend_from_slice(&[0x00, 0x00]);
    let rect = Rect::new(0, 0, 32, 16);
    let outcome = harness
        .decode(ENCODING_HEXTILE, rect, &payload)
        .expect("a well formed hextile rect decodes");
    assert_eq!(
        outcome,
        RectOutcome::Pixels(rect),
        "the outcome names the painted rect"
    );
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(10, 20, 30),
        "the first tile is filled with its background"
    );
    assert_eq!(
        harness.pixel(3, 2),
        rgba_pixel(200, 0, 0),
        "the coloured subrect starts at its tile offset"
    );
    assert_eq!(
        harness.pixel(6, 3),
        rgba_pixel(200, 0, 0),
        "the coloured subrect is four wide and two tall"
    );
    assert_eq!(
        harness.pixel(7, 2),
        rgba_pixel(10, 20, 30),
        "the pixel right of the subrect shows the background"
    );
    assert_eq!(
        harness.pixel(3, 4),
        rgba_pixel(10, 20, 30),
        "the pixel below the subrect shows the background"
    );
    assert_eq!(
        harness.pixel(16, 0),
        rgba_pixel(0, 200, 0),
        "the second tile paints its own coloured subrect"
    );
    assert_eq!(
        harness.pixel(17, 0),
        rgba_pixel(10, 20, 30),
        "the second tile reuses the background of the first"
    );
    assert_eq!(
        harness.pixel(31, 15),
        rgba_pixel(10, 20, 30),
        "the whole second tile uses the remembered background"
    );
}
