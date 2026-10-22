use rfb::{ENCODING_CURSOR, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_cursor_larger_than_the_cap_is_refused() {
    for (width, height) in [(513u16, 1u16), (1, 513), (513, 513), (u16::MAX, u16::MAX)] {
        let mut harness = DecodeHarness::new(4, 4);
        let (outcome, consumed) =
            harness.decode_counting(ENCODING_CURSOR, Rect::new(0, 0, width, height), &[0u8; 64]);
        assert!(
            matches!(outcome, Err(DecodeError::Corrupt(_))),
            "a {width}x{height} cursor is over the cap and refused, got {outcome:?}"
        );
        assert_eq!(
            consumed, 0,
            "nothing is read for the oversized {width}x{height} cursor"
        );
    }
}
