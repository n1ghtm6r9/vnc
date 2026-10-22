use rfb::{ENCODING_ZLIB, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture, length_prefixed};

#[test]
fn test_zlib_data_inflating_short_of_the_rect_is_refused() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut fixture = ZlibFixture::new();
    let payload = length_prefixed(&fixture.compress(&[7u8; 8]));
    let result = harness.decode(ENCODING_ZLIB, Rect::new(0, 0, 2, 2), &payload);
    assert!(
        matches!(result, Err(DecodeError::Corrupt(_))),
        "data that inflates short of the rect size is corrupt, got {result:?}"
    );
}
