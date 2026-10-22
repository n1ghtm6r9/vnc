use rfb::{
    ENCODING_APPLE_MVS, ENCODING_COPY_RECT, ENCODING_HEXTILE, ENCODING_RAW, ENCODING_RRE,
    ENCODING_TIGHT, ENCODING_ZLIB, ENCODING_ZRLE,
};

pub(crate) fn encoding_name(encoding: i32) -> String {
    match encoding {
        ENCODING_RAW => "Raw".to_owned(),
        ENCODING_COPY_RECT => "CopyRect".to_owned(),
        ENCODING_RRE => "RRE".to_owned(),
        ENCODING_HEXTILE => "Hextile".to_owned(),
        ENCODING_ZLIB => "Zlib".to_owned(),
        ENCODING_TIGHT => "Tight".to_owned(),
        ENCODING_ZRLE => "ZRLE".to_owned(),
        ENCODING_APPLE_MVS => "Apple MVS".to_owned(),
        other => format!("encoding {other}"),
    }
}
