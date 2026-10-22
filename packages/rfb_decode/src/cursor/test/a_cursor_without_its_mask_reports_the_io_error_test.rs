use rfb::{ENCODING_CURSOR, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_cursor_without_its_mask_reports_the_io_error() {
    let mut harness = DecodeHarness::new(4, 4);
    let (outcome, _) = harness.decode_counting(ENCODING_CURSOR, Rect::new(0, 0, 2, 2), &[0u8; 16]);
    assert!(
        matches!(outcome, Err(DecodeError::Io(_))),
        "a cursor cut off before its mask is an I/O error, got {outcome:?}"
    );
}
