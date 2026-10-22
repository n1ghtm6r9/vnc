use rfb::{ENCODING_ZRLE, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_zrle_length_larger_than_the_rect_allows_is_refused() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut payload = 0x7FFF_FFFFu32.to_be_bytes().to_vec();
    payload.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let (outcome, consumed) =
        harness.decode_counting(ENCODING_ZRLE, Rect::new(0, 0, 2, 2), &payload);
    assert!(
        matches!(outcome, Err(DecodeError::Corrupt(_))),
        "a length no valid stream needs is corrupt, got {outcome:?}"
    );
    assert_eq!(consumed, 4, "only the length was read before the refusal");
}
