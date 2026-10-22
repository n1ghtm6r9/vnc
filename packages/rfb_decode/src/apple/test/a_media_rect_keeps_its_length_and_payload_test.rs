use rfb::{ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MEDIA_STREAM, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_media_rect_keeps_its_length_and_payload() {
    for encoding in [ENCODING_APPLE_MEDIA_STREAM, ENCODING_APPLE_HIGH_PERFORMANCE] {
        let mut harness = DecodeHarness::new(4, 4);

        let outcome = harness
            .decode(encoding, Rect::new(0, 0, 0, 0), &[0, 3, 7, 8, 9])
            .expect("a media rect decodes");

        assert_eq!(
            outcome,
            RectOutcome::AppleMedia {
                encoding,
                payload: vec![0, 3, 7, 8, 9]
            },
            "encoding {encoding} hands over its payload with the length in front, as the media negotiation reads it"
        );
    }
}
