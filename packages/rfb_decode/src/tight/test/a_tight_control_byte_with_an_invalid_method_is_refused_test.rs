use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_tight_control_byte_with_an_invalid_method_is_refused() {
    for control in [0xA0u8, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xAF] {
        let mut harness = DecodeHarness::new(4, 4);
        let mut payload = vec![control];
        payload.extend_from_slice(&[0u8; 64]);
        let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 2, 2), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "control byte {control:#04X} is not a method this client accepts, got {result:?}"
        );
    }
}
