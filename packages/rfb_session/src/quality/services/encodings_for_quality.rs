use rfb::{
    ENCODING_APPLE_MVS, ENCODING_COMPRESS_LEVEL_0, ENCODING_COPY_RECT, ENCODING_HEXTILE,
    ENCODING_JPEG_QUALITY_LEVEL_0, ENCODING_RAW, ENCODING_RRE, ENCODING_TIGHT, ENCODING_ZLIB,
    ENCODING_ZRLE,
};

use crate::{PSEUDO_ENCODINGS, Quality};

pub fn encodings_for_quality(quality: Quality) -> Vec<i32> {
    let mut encodings = match quality {
        Quality::Lossless => vec![
            ENCODING_COPY_RECT,
            ENCODING_ZRLE,
            ENCODING_HEXTILE,
            ENCODING_ZLIB,
            ENCODING_RRE,
            ENCODING_RAW,
        ],
        Quality::Balanced => vec![
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
        Quality::Fast => vec![
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
    };
    encodings.extend(PSEUDO_ENCODINGS);
    encodings
}
