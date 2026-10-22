use rfb::{
    ENCODING_APPLE_MVS, ENCODING_COPY_RECT, ENCODING_HEXTILE, ENCODING_RAW, ENCODING_RRE,
    ENCODING_TIGHT, ENCODING_ZLIB, ENCODING_ZRLE, Rect,
};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_rect_outside_the_framebuffer_is_refused() {
    let rects = [
        Rect::new(10, 10, 10, 10),
        Rect::new(0, 0, 17, 1),
        Rect::new(0, 16, 1, 1),
        Rect::new(65535, 65535, 65535, 65535),
    ];
    let encodings = [
        ENCODING_RAW,
        ENCODING_COPY_RECT,
        ENCODING_RRE,
        ENCODING_HEXTILE,
        ENCODING_ZLIB,
        ENCODING_ZRLE,
        ENCODING_TIGHT,
        ENCODING_APPLE_MVS,
    ];
    for encoding in encodings {
        for rect in rects {
            let mut harness = DecodeHarness::new(16, 16);
            let (outcome, consumed) = harness.decode_counting(encoding, rect, &[0xAB; 64]);
            match outcome {
                Err(DecodeError::RectOutOfBounds(reported)) => assert_eq!(
                    reported, rect,
                    "the error names the offending rect for encoding {encoding}"
                ),
                other => panic!("encoding {encoding} must refuse {rect:?}, got {other:?}"),
            }
            assert_eq!(
                consumed, 0,
                "no payload byte is read for encoding {encoding} and {rect:?}"
            );
        }
    }
}
