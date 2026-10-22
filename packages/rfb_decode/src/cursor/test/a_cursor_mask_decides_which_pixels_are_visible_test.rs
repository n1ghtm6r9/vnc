use rfb::{ENCODING_CURSOR, Rect};

use crate::test_support::{DecodeHarness, wire_pixel};
use crate::{CursorShape, RectOutcome};

#[test]
fn test_a_cursor_mask_decides_which_pixels_are_visible() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut payload = Vec::new();
    for (red, green, blue) in [
        (10, 20, 30),
        (40, 50, 60),
        (70, 80, 90),
        (100, 110, 120),
        (130, 140, 150),
        (160, 170, 180),
    ] {
        payload.extend_from_slice(&wire_pixel(red, green, blue));
    }
    payload.extend_from_slice(&[0b1010_0000, 0b0100_0000]);
    let outcome = harness
        .decode(ENCODING_CURSOR, Rect::new(2, 3, 3, 2), &payload)
        .expect("a cursor decodes");
    let expected = CursorShape {
        width: 3,
        height: 2,
        hotspot_x: 2,
        hotspot_y: 3,
        rgba: vec![
            10, 20, 30, 255, 0, 0, 0, 0, 70, 80, 90, 255, 0, 0, 0, 0, 130, 140, 150, 255, 0, 0, 0,
            0,
        ],
    };
    assert_eq!(
        outcome,
        RectOutcome::Cursor(expected),
        "the mask keeps the pixels whose bit is set and clears the others"
    );
}
