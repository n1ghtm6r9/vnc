use rfb::{ENCODING_HEXTILE, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_hextile_subrect_outside_its_tile_is_refused() {
    let mut harness = DecodeHarness::new(16, 16);
    let past_the_full_tile = harness.decode(
        ENCODING_HEXTILE,
        Rect::new(0, 0, 16, 16),
        &[8, 1, 0xF0, 0x10],
    );
    assert!(
        matches!(past_the_full_tile, Err(DecodeError::Corrupt(_))),
        "a subrect past a 16 wide tile is corrupt, got {past_the_full_tile:?}"
    );

    let past_the_clipped_tile =
        harness.decode(ENCODING_HEXTILE, Rect::new(0, 0, 4, 4), &[8, 1, 0x00, 0x44]);
    assert!(
        matches!(past_the_clipped_tile, Err(DecodeError::Corrupt(_))),
        "a subrect past a clipped tile is corrupt, got {past_the_clipped_tile:?}"
    );
}
