use rfb::{ENCODING_ZRLE, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel};

#[test]
fn test_a_zrle_tile_cut_short_is_refused() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut tile = vec![0u8];
    for index in 0..5u8 {
        tile.extend_from_slice(&compact_pixel(index, index, index));
    }
    let payload = ZlibFixture::new().payload(&tile);
    let result = harness.decode(ENCODING_ZRLE, Rect::new(0, 0, 4, 4), &payload);
    assert!(
        matches!(result, Err(DecodeError::Corrupt(_))),
        "a raw tile with five of sixteen pixels is corrupt, got {result:?}"
    );
}
