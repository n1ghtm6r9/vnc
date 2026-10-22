use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::test_support::{DecodeHarness, apple_cursor_payload};
use crate::{RectOutcome, SystemCursor};

#[test]
fn test_the_predefined_apple_cursors_are_system_cursors() {
    let expected = [
        (0, SystemCursor::Default),
        (1, SystemCursor::Text),
        (2, SystemCursor::Crosshair),
        (3, SystemCursor::Grabbing),
        (4, SystemCursor::Grab),
        (5, SystemCursor::Pointer),
        (6, SystemCursor::Wait),
        (7, SystemCursor::Help),
        (8, SystemCursor::NotAllowed),
        (9, SystemCursor::Move),
        (10, SystemCursor::NsResize),
        (11, SystemCursor::EwResize),
        (999, SystemCursor::Default),
    ];
    for (id, cursor) in expected {
        let mut harness = DecodeHarness::new(4, 4);
        let outcome = harness
            .decode(
                ENCODING_APPLE_CURSOR_ALPHA,
                Rect::new(0, 0, 16, 16),
                &apple_cursor_payload(id, &[9, 9]),
            )
            .expect("a predefined cursor decodes");
        assert_eq!(
            outcome,
            RectOutcome::SystemCursor(cursor),
            "apple cursor {id} is the system cursor {cursor:?}, and its bytes are skipped"
        );
    }
}
