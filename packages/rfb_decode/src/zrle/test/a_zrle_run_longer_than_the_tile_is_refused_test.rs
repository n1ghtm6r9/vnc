use rfb::{ENCODING_ZRLE, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel};

#[test]
fn test_a_zrle_run_longer_than_the_tile_is_refused() {
    let red = compact_pixel(255, 0, 0);
    let blue = compact_pixel(0, 0, 255);
    let mut one_long_run = vec![128u8];
    one_long_run.extend_from_slice(&red);
    one_long_run.extend_from_slice(&[255, 10]);
    let mut two_runs_overflowing = vec![128u8];
    two_runs_overflowing.extend_from_slice(&red);
    two_runs_overflowing.push(9);
    two_runs_overflowing.extend_from_slice(&blue);
    two_runs_overflowing.push(6);
    let mut palette_run = vec![130u8];
    palette_run.extend_from_slice(&red);
    palette_run.extend_from_slice(&blue);
    palette_run.extend_from_slice(&[0x80, 20]);
    for (name, tile) in [
        ("one run past the tile", one_long_run),
        ("two runs adding up past the tile", two_runs_overflowing),
        ("a palette run past the tile", palette_run),
    ] {
        let mut harness = DecodeHarness::new(4, 4);
        let payload = ZlibFixture::new().payload(&tile);
        let result = harness.decode(ENCODING_ZRLE, Rect::new(0, 0, 4, 4), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "{name} must be refused, got {result:?}"
        );
    }
}
