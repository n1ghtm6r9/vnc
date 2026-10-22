use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_tight_filter_that_is_unknown_is_refused() {
    for filter in [3u8, 4, 100, 255] {
        let mut harness = DecodeHarness::new(4, 4);
        let mut payload = vec![0x40u8, filter];
        payload.extend_from_slice(&[0u8; 64]);
        let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 2, 2), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "filter {filter} is not defined, got {result:?}"
        );
    }
}
