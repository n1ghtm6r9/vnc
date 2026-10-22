use rfb::{ENCODING_APPLE_CURSOR_POSITION, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_the_apple_cursor_position_is_ignored() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(ENCODING_APPLE_CURSOR_POSITION, Rect::new(30, 40, 0, 0), &[])
        .expect("a cursor position decodes");
    assert_eq!(
        outcome,
        RectOutcome::Ignored,
        "the client draws its own cursor, so where the Mac puts it does not matter"
    );
}
