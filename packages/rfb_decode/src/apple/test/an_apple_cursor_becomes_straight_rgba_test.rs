use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::test_support::{DecodeHarness, apple_cursor_payload, finished_zlib_stream};
use crate::{CursorShape, RectOutcome};

#[test]
fn test_an_apple_cursor_becomes_straight_rgba() {
    let raw = [10, 20, 30, 0, 40, 50, 60, 0, 255, 128];
    let payload = apple_cursor_payload(1000, &finished_zlib_stream(&raw));
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(ENCODING_APPLE_CURSOR_ALPHA, Rect::new(1, 0, 2, 1), &payload)
        .expect("an apple cursor decodes");
    let shape = CursorShape {
        width: 2,
        height: 1,
        hotspot_x: 1,
        hotspot_y: 0,
        rgba: vec![30, 20, 10, 255, 60, 50, 40, 128],
    };
    assert_eq!(
        outcome,
        RectOutcome::Cursor(shape),
        "blue-green-red pixels and a separate alpha plane become red-green-blue-alpha, with the rect corner as the hotspot"
    );
}
