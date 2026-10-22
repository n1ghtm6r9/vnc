use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, compact_length, jpeg_of_rgb};

#[test]
fn test_a_tight_jpeg_of_the_wrong_size_is_refused() {
    let jpeg = jpeg_of_rgb(8, 8, &[90u8; 8 * 8 * 3]);
    let mut payload = vec![0x90u8];
    payload.extend_from_slice(&compact_length(jpeg.len()));
    payload.extend_from_slice(&jpeg);
    for rect in [
        Rect::new(0, 0, 16, 16),
        Rect::new(0, 0, 8, 4),
        Rect::new(0, 0, 4, 8),
    ] {
        let mut harness = DecodeHarness::new(16, 16);
        let result = harness.decode(ENCODING_TIGHT, rect, &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "an 8x8 JPEG in a {}x{} rect is refused, got {result:?}",
            rect.width,
            rect.height
        );
    }
}
