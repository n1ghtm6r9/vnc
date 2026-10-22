use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, compact_pixel};

#[test]
fn test_a_tight_palette_index_outside_the_palette_is_refused() {
    for bad_index in [3u8, 4, 255] {
        let mut harness = DecodeHarness::new(4, 1);
        let mut payload = vec![0x40u8, 1, 2];
        for colour in [(1, 2, 3), (4, 5, 6), (7, 8, 9)] {
            payload.extend_from_slice(&compact_pixel(colour.0, colour.1, colour.2));
        }
        payload.extend_from_slice(&[0, 1, 2, bad_index]);
        let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 1), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "palette index {bad_index} of a three colour palette is refused, got {result:?}"
        );
    }
}
