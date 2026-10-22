use std::io::Read;

use rfb::{
    ENCODING_APPLE_CURSOR_ALPHA, ENCODING_APPLE_CURSOR_POSITION, ENCODING_APPLE_DEVICE_INFO,
    ENCODING_APPLE_DISPLAY_INFO, ENCODING_APPLE_DISPLAY_INFO_2, ENCODING_APPLE_HIGH_PERFORMANCE,
    ENCODING_APPLE_KEYBOARD_INPUT, ENCODING_APPLE_MEDIA_STREAM, ENCODING_APPLE_MVS,
    ENCODING_APPLE_USER_INFO, ENCODING_COPY_RECT, ENCODING_CURSOR, ENCODING_DESKTOP_SIZE,
    ENCODING_EXTENDED_DESKTOP_SIZE, ENCODING_HEXTILE, ENCODING_LAST_RECT, ENCODING_RAW,
    ENCODING_RRE, ENCODING_TIGHT, ENCODING_ZLIB, ENCODING_ZRLE, RectHeader,
};

use crate::{
    DecodeError, DecodeTarget, RectOutcome, decode_apple_cursor, decode_apple_display_info,
    decode_copy_rect, decode_cursor, decode_desktop_size, decode_extended_desktop_size,
    decode_hextile, decode_mvs, decode_raw, decode_rre, decode_tight, decode_zlib, decode_zrle,
    read_apple_keyboard_input, read_apple_media, skip_apple_display_layout, skip_apple_info_block,
    skip_apple_user_info,
};

pub fn decode_rect(
    reader: &mut impl Read,
    header: &RectHeader,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let rect = header.rect;
    match header.encoding {
        ENCODING_RAW => decode_raw(reader, rect, target),
        ENCODING_COPY_RECT => decode_copy_rect(reader, rect, target),
        ENCODING_RRE => decode_rre(reader, rect, target),
        ENCODING_HEXTILE => decode_hextile(reader, rect, target),
        ENCODING_ZLIB => decode_zlib(reader, rect, target),
        ENCODING_TIGHT => decode_tight(reader, rect, target),
        ENCODING_ZRLE => decode_zrle(reader, rect, target),
        ENCODING_APPLE_MVS => decode_mvs(reader, rect, target),
        ENCODING_CURSOR => decode_cursor(reader, rect, target),
        ENCODING_DESKTOP_SIZE => decode_desktop_size(rect, target),
        ENCODING_EXTENDED_DESKTOP_SIZE => decode_extended_desktop_size(reader, rect, target),
        ENCODING_LAST_RECT => Ok(RectOutcome::LastRect),
        ENCODING_APPLE_CURSOR_POSITION => Ok(RectOutcome::Ignored),
        ENCODING_APPLE_DISPLAY_INFO => skip_apple_display_layout(reader),
        ENCODING_APPLE_USER_INFO => skip_apple_user_info(reader),
        ENCODING_APPLE_CURSOR_ALPHA => decode_apple_cursor(reader, rect, target),
        ENCODING_APPLE_DISPLAY_INFO_2 => decode_apple_display_info(reader, rect, target),
        ENCODING_APPLE_DEVICE_INFO => skip_apple_info_block(reader),
        ENCODING_APPLE_KEYBOARD_INPUT => read_apple_keyboard_input(reader),
        ENCODING_APPLE_HIGH_PERFORMANCE | ENCODING_APPLE_MEDIA_STREAM => {
            read_apple_media(reader, header.encoding)
        }
        other => Err(DecodeError::UnsupportedEncoding(other)),
    }
}
