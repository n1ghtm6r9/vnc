use rfb::{
    ENCODING_APPLE_MVS, ENCODING_COMPRESS_LEVEL_0, ENCODING_COPY_RECT, ENCODING_CURSOR,
    ENCODING_DESKTOP_SIZE, ENCODING_EXTENDED_DESKTOP_SIZE, ENCODING_HEXTILE,
    ENCODING_JPEG_QUALITY_LEVEL_0, ENCODING_LAST_RECT, ENCODING_RAW, ENCODING_RRE, ENCODING_TIGHT,
    ENCODING_ZLIB, ENCODING_ZRLE,
};

use crate::{Quality, encodings_for_quality};

#[test]
fn test_each_quality_asks_for_its_encodings_in_preference_order() {
    let pseudo_encodings = [
        ENCODING_CURSOR,
        ENCODING_DESKTOP_SIZE,
        ENCODING_EXTENDED_DESKTOP_SIZE,
        ENCODING_LAST_RECT,
    ];
    let expected = [
        (
            Quality::Lossless,
            vec![
                ENCODING_COPY_RECT,
                ENCODING_ZRLE,
                ENCODING_HEXTILE,
                ENCODING_ZLIB,
                ENCODING_RRE,
                ENCODING_RAW,
            ],
        ),
        (
            Quality::Balanced,
            vec![
                ENCODING_APPLE_MVS,
                ENCODING_COPY_RECT,
                ENCODING_TIGHT,
                ENCODING_ZRLE,
                ENCODING_HEXTILE,
                ENCODING_ZLIB,
                ENCODING_RRE,
                ENCODING_RAW,
                ENCODING_JPEG_QUALITY_LEVEL_0 + 8,
                ENCODING_COMPRESS_LEVEL_0 + 6,
            ],
        ),
        (
            Quality::Fast,
            vec![
                ENCODING_APPLE_MVS,
                ENCODING_COPY_RECT,
                ENCODING_TIGHT,
                ENCODING_ZRLE,
                ENCODING_HEXTILE,
                ENCODING_ZLIB,
                ENCODING_RRE,
                ENCODING_RAW,
                ENCODING_JPEG_QUALITY_LEVEL_0 + 5,
                ENCODING_COMPRESS_LEVEL_0 + 2,
            ],
        ),
    ];
    for (quality, mut encodings) in expected {
        encodings.extend(pseudo_encodings);
        assert_eq!(
            encodings_for_quality(quality),
            encodings,
            "{quality:?} quality asks for its encodings best first, then for the pseudo-encodings"
        );
    }
}
