use std::mem::{offset_of, size_of};

use crate::CuvidVideoFormat;

#[test]
fn test_the_video_format_matches_the_c_layout() {
    let fields = [
        ("codec", offset_of!(CuvidVideoFormat, codec), 0),
        (
            "frame_rate_numerator",
            offset_of!(CuvidVideoFormat, frame_rate_numerator),
            4,
        ),
        (
            "frame_rate_denominator",
            offset_of!(CuvidVideoFormat, frame_rate_denominator),
            8,
        ),
        (
            "progressive_sequence",
            offset_of!(CuvidVideoFormat, progressive_sequence),
            12,
        ),
        (
            "bit_depth_luma_minus8",
            offset_of!(CuvidVideoFormat, bit_depth_luma_minus8),
            13,
        ),
        (
            "bit_depth_chroma_minus8",
            offset_of!(CuvidVideoFormat, bit_depth_chroma_minus8),
            14,
        ),
        (
            "min_num_decode_surfaces",
            offset_of!(CuvidVideoFormat, min_num_decode_surfaces),
            15,
        ),
        ("coded_width", offset_of!(CuvidVideoFormat, coded_width), 16),
        (
            "coded_height",
            offset_of!(CuvidVideoFormat, coded_height),
            20,
        ),
        (
            "display_left",
            offset_of!(CuvidVideoFormat, display_left),
            24,
        ),
        ("display_top", offset_of!(CuvidVideoFormat, display_top), 28),
        (
            "display_right",
            offset_of!(CuvidVideoFormat, display_right),
            32,
        ),
        (
            "display_bottom",
            offset_of!(CuvidVideoFormat, display_bottom),
            36,
        ),
        (
            "chroma_format",
            offset_of!(CuvidVideoFormat, chroma_format),
            40,
        ),
        ("bitrate", offset_of!(CuvidVideoFormat, bitrate), 44),
        (
            "display_aspect_ratio_x",
            offset_of!(CuvidVideoFormat, display_aspect_ratio_x),
            48,
        ),
        (
            "display_aspect_ratio_y",
            offset_of!(CuvidVideoFormat, display_aspect_ratio_y),
            52,
        ),
        (
            "video_signal_flags",
            offset_of!(CuvidVideoFormat, video_signal_flags),
            56,
        ),
        (
            "colour_primaries",
            offset_of!(CuvidVideoFormat, colour_primaries),
            57,
        ),
        (
            "transfer_characteristics",
            offset_of!(CuvidVideoFormat, transfer_characteristics),
            58,
        ),
        (
            "matrix_coefficients",
            offset_of!(CuvidVideoFormat, matrix_coefficients),
            59,
        ),
        (
            "seqhdr_data_length",
            offset_of!(CuvidVideoFormat, seqhdr_data_length),
            60,
        ),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(actual, expected, "{name} sits where CUVIDEOFORMAT keeps it");
    }
    assert_eq!(
        size_of::<CuvidVideoFormat>(),
        64,
        "the format is as big as CUVIDEOFORMAT"
    );
}
