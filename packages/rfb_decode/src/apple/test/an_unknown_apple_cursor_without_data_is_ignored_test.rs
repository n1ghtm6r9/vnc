use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, apple_cursor_payload};

#[test]
fn test_an_unknown_apple_cursor_without_data_is_ignored() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(
            ENCODING_APPLE_CURSOR_ALPHA,
            Rect::new(0, 0, 1, 1),
            &apple_cursor_payload(1002, &[]),
        )
        .expect("a cursor without data decodes");
    assert_eq!(
        outcome,
        RectOutcome::Ignored,
        "a cursor never seen cannot be shown, so the current one stays"
    );
}
