use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_an_apple_cursor_claiming_too_many_bytes_is_refused() {
    let mut payload = 1000u32.to_be_bytes().to_vec();
    payload.extend_from_slice(&u32::MAX.to_be_bytes());
    let mut harness = DecodeHarness::new(4, 4);
    let error = harness
        .decode(ENCODING_APPLE_CURSOR_ALPHA, Rect::new(0, 0, 1, 1), &payload)
        .unwrap_err();
    assert!(
        matches!(error, DecodeError::Corrupt(_)),
        "a cursor of 4 GB cannot be real, got {error:?}"
    );
}
