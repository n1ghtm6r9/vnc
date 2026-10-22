use rfb::Rect;

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_an_unknown_encoding_is_refused() {
    let mut harness = DecodeHarness::new(4, 4);
    let (outcome, consumed) = harness.decode_counting(1234, Rect::new(0, 0, 2, 2), &[1, 2, 3, 4]);
    assert!(
        matches!(outcome, Err(DecodeError::UnsupportedEncoding(1234))),
        "an encoding the client never asked for is refused, got {outcome:?}"
    );
    assert_eq!(consumed, 0, "nothing is read for an unknown encoding");
}
