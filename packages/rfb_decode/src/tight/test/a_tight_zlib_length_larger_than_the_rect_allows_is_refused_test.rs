use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_tight_zlib_length_larger_than_the_rect_allows_is_refused() {
    let mut harness = DecodeHarness::new(4, 3);
    let mut payload = vec![0x00u8, 0xFF, 0xFF, 0xFF];
    payload.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let (outcome, consumed) =
        harness.decode_counting(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &payload);
    assert!(
        matches!(outcome, Err(DecodeError::Corrupt(_))),
        "a length no valid stream needs is corrupt, got {outcome:?}"
    );
    assert_eq!(
        consumed, 4,
        "only the control byte and the length were read before the refusal"
    );
}
