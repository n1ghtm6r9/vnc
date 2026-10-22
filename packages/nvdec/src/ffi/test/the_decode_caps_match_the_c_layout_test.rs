use std::mem::{offset_of, size_of};

use crate::CuvidDecodeCaps;

#[test]
fn test_the_decode_caps_match_the_c_layout() {
    let fields = [
        ("codec_type", offset_of!(CuvidDecodeCaps, codec_type), 0),
        (
            "chroma_format",
            offset_of!(CuvidDecodeCaps, chroma_format),
            4,
        ),
        (
            "bit_depth_minus8",
            offset_of!(CuvidDecodeCaps, bit_depth_minus8),
            8,
        ),
        (
            "is_supported",
            offset_of!(CuvidDecodeCaps, is_supported),
            24,
        ),
        ("num_nvdecs", offset_of!(CuvidDecodeCaps, num_nvdecs), 25),
        (
            "output_format_mask",
            offset_of!(CuvidDecodeCaps, output_format_mask),
            26,
        ),
        ("max_width", offset_of!(CuvidDecodeCaps, max_width), 28),
        ("max_height", offset_of!(CuvidDecodeCaps, max_height), 32),
        (
            "max_mb_count",
            offset_of!(CuvidDecodeCaps, max_mb_count),
            36,
        ),
        ("min_width", offset_of!(CuvidDecodeCaps, min_width), 40),
        ("min_height", offset_of!(CuvidDecodeCaps, min_height), 42),
        (
            "is_histogram_supported",
            offset_of!(CuvidDecodeCaps, is_histogram_supported),
            44,
        ),
        (
            "max_histogram_bins",
            offset_of!(CuvidDecodeCaps, max_histogram_bins),
            46,
        ),
        (
            "is_decode_stats_supported",
            offset_of!(CuvidDecodeCaps, is_decode_stats_supported),
            48,
        ),
        ("reserved3", offset_of!(CuvidDecodeCaps, reserved3), 52),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(
            actual, expected,
            "{name} sits where CUVIDDECODECAPS keeps it"
        );
    }
    assert_eq!(
        size_of::<CuvidDecodeCaps>(),
        88,
        "the caps are as big as CUVIDDECODECAPS"
    );
}
