use rfb::{ENCODING_CURSOR, Rect};

use crate::test_support::DecodeHarness;
use crate::{CursorShape, RectOutcome};

#[test]
fn test_an_empty_cursor_hides_the_cursor() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(ENCODING_CURSOR, Rect::new(5, 6, 0, 0), &[])
        .expect("an empty cursor decodes without any data");
    let hidden = CursorShape {
        width: 0,
        height: 0,
        hotspot_x: 5,
        hotspot_y: 6,
        rgba: Vec::new(),
    };
    assert_eq!(
        outcome,
        RectOutcome::Cursor(hidden),
        "a cursor without pixels has no image"
    );
}
