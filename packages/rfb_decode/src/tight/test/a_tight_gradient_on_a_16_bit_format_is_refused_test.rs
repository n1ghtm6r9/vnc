use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, rgb565_big_endian};

#[test]
fn test_a_tight_gradient_on_a_16_bit_format_is_refused() {
    let mut harness = DecodeHarness::with_format(4, 4, &rgb565_big_endian());
    let mut payload = vec![0x40u8, 2];
    payload.extend_from_slice(&[0u8; 64]);
    let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 2, 2), &payload);
    assert!(
        matches!(result, Err(DecodeError::Corrupt(_))),
        "the gradient filter only works on 24 bit pixels, got {result:?}"
    );
}
