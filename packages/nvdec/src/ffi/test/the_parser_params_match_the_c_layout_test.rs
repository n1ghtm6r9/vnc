use std::mem::{offset_of, size_of};

use crate::CuvidParserParams;

#[test]
fn test_the_parser_params_match_the_c_layout() {
    let fields = [
        ("codec_type", offset_of!(CuvidParserParams, codec_type), 0),
        (
            "max_num_decode_surfaces",
            offset_of!(CuvidParserParams, max_num_decode_surfaces),
            4,
        ),
        ("clock_rate", offset_of!(CuvidParserParams, clock_rate), 8),
        (
            "error_threshold",
            offset_of!(CuvidParserParams, error_threshold),
            12,
        ),
        (
            "max_display_delay",
            offset_of!(CuvidParserParams, max_display_delay),
            16,
        ),
        ("flags", offset_of!(CuvidParserParams, flags), 20),
        ("reserved1", offset_of!(CuvidParserParams, reserved1), 24),
        ("user_data", offset_of!(CuvidParserParams, user_data), 40),
        (
            "sequence_callback",
            offset_of!(CuvidParserParams, sequence_callback),
            48,
        ),
        (
            "decode_callback",
            offset_of!(CuvidParserParams, decode_callback),
            56,
        ),
        (
            "display_callback",
            offset_of!(CuvidParserParams, display_callback),
            64,
        ),
        (
            "operating_point_callback",
            offset_of!(CuvidParserParams, operating_point_callback),
            72,
        ),
        (
            "sei_message_callback",
            offset_of!(CuvidParserParams, sei_message_callback),
            80,
        ),
        ("reserved2", offset_of!(CuvidParserParams, reserved2), 88),
        (
            "ext_video_info",
            offset_of!(CuvidParserParams, ext_video_info),
            128,
        ),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(
            actual, expected,
            "{name} sits where CUVIDPARSERPARAMS keeps it"
        );
    }
    assert_eq!(
        size_of::<CuvidParserParams>(),
        136,
        "the parser params are as big as CUVIDPARSERPARAMS"
    );
}
