use rfb::{
    ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MVS, ENCODING_COPY_RECT, ENCODING_HEXTILE, ENCODING_RAW, ENCODING_RRE,
    ENCODING_TIGHT, ENCODING_ZLIB, ENCODING_ZRLE,
};

pub fn encoding_name(encoding: i32) -> String {
    let name = match encoding {
        ENCODING_RAW => "Raw",
        ENCODING_COPY_RECT => "CopyRect",
        ENCODING_RRE => "RRE",
        ENCODING_HEXTILE => "Hextile",
        ENCODING_ZLIB => "Zlib",
        ENCODING_TIGHT => "Tight",
        ENCODING_ZRLE => "ZRLE",
        ENCODING_APPLE_MVS => "Apple MVS",
        ENCODING_APPLE_HIGH_PERFORMANCE => "High Performance",
        other => return other.to_string(),
    };
    name.to_string()
}
