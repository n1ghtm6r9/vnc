use rfb::{ENCODING_ZRLE, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture};

#[test]
fn test_zrle_data_inflating_beyond_the_rect_is_refused() {
    let mut harness = DecodeHarness::new(4, 4);
    let payload = ZlibFixture::new().payload(&[0u8; 65536]);
    let result = harness.decode(ENCODING_ZRLE, Rect::new(0, 0, 2, 2), &payload);
    assert!(
        matches!(result, Err(DecodeError::Corrupt(_))),
        "data that inflates past any tile layout of the rect is a bomb, got {result:?}"
    );
}
