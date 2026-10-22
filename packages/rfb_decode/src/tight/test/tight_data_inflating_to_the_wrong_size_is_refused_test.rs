use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture};

#[test]
fn test_tight_data_inflating_to_the_wrong_size_is_refused() {
    for (name, inflated) in [
        ("beyond the rect", vec![0u8; 65536]),
        ("short of the rect", vec![7u8; 20]),
        ("by a single byte", vec![7u8; 37]),
    ] {
        let mut harness = DecodeHarness::new(4, 3);
        let mut payload = vec![0x00u8];
        payload.extend_from_slice(&ZlibFixture::new().tight_payload(&inflated));
        let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "data inflating {name} is refused, got {result:?}"
        );
    }
}
