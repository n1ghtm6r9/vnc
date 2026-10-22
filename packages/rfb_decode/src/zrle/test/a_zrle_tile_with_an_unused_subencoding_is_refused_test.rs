use rfb::{ENCODING_ZRLE, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture};

#[test]
fn test_a_zrle_tile_with_an_unused_subencoding_is_refused() {
    for subencoding in [17u8, 50, 100, 127, 129] {
        let mut harness = DecodeHarness::new(4, 4);
        let mut tile = vec![subencoding];
        tile.extend_from_slice(&[0u8; 200]);
        let payload = ZlibFixture::new().payload(&tile);
        let result = harness.decode(ENCODING_ZRLE, Rect::new(0, 0, 4, 4), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "subencoding {subencoding} is unused and must be refused, got {result:?}"
        );
    }
}
