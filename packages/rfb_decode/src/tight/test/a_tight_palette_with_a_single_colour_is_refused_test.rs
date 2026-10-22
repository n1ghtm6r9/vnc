use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, compact_pixel};

#[test]
fn test_a_tight_palette_with_a_single_colour_is_refused() {
    let mut harness = DecodeHarness::new(4, 1);
    let mut payload = vec![0x40u8, 1, 0];
    payload.extend_from_slice(&compact_pixel(1, 2, 3));
    payload.extend_from_slice(&[0u8; 8]);
    let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 1), &payload);
    assert!(
        matches!(result, Err(DecodeError::Corrupt(_))),
        "a palette needs at least two colours, got {result:?}"
    );
}
