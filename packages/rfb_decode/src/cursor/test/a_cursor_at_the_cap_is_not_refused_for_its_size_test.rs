use rfb::{ENCODING_CURSOR, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_cursor_at_the_cap_is_not_refused_for_its_size() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness.decode(ENCODING_CURSOR, Rect::new(0, 0, 512, 512), &[0u8; 64]);
    assert!(
        matches!(outcome, Err(DecodeError::Io(_))),
        "a 512x512 cursor is allowed and only fails on the missing data, got {outcome:?}"
    );
}
