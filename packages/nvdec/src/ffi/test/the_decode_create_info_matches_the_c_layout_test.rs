use std::mem::{offset_of, size_of};

use crate::CuvidDecodeCreateInfo;

#[test]
fn test_the_decode_create_info_matches_the_c_layout() {
    let fields = [
        ("width", offset_of!(CuvidDecodeCreateInfo, width), 0),
        ("height", offset_of!(CuvidDecodeCreateInfo, height), 4),
        (
            "num_decode_surfaces",
            offset_of!(CuvidDecodeCreateInfo, num_decode_surfaces),
            8,
        ),
        (
            "codec_type",
            offset_of!(CuvidDecodeCreateInfo, codec_type),
            12,
        ),
        (
            "chroma_format",
            offset_of!(CuvidDecodeCreateInfo, chroma_format),
            16,
        ),
        (
            "creation_flags",
            offset_of!(CuvidDecodeCreateInfo, creation_flags),
            20,
        ),
        (
            "bit_depth_minus8",
            offset_of!(CuvidDecodeCreateInfo, bit_depth_minus8),
            24,
        ),
        (
            "intra_decode_only",
            offset_of!(CuvidDecodeCreateInfo, intra_decode_only),
            28,
        ),
        (
            "max_width",
            offset_of!(CuvidDecodeCreateInfo, max_width),
            32,
        ),
        (
            "max_height",
            offset_of!(CuvidDecodeCreateInfo, max_height),
            36,
        ),
        (
            "display_area",
            offset_of!(CuvidDecodeCreateInfo, display_area),
            44,
        ),
        (
            "output_format",
            offset_of!(CuvidDecodeCreateInfo, output_format),
            52,
        ),
        (
            "deinterlace_mode",
            offset_of!(CuvidDecodeCreateInfo, deinterlace_mode),
            56,
        ),
        (
            "target_width",
            offset_of!(CuvidDecodeCreateInfo, target_width),
            60,
        ),
        (
            "target_height",
            offset_of!(CuvidDecodeCreateInfo, target_height),
            64,
        ),
        (
            "num_output_surfaces",
            offset_of!(CuvidDecodeCreateInfo, num_output_surfaces),
            68,
        ),
        ("vid_lock", offset_of!(CuvidDecodeCreateInfo, vid_lock), 72),
        (
            "target_rect",
            offset_of!(CuvidDecodeCreateInfo, target_rect),
            80,
        ),
        (
            "enable_histogram",
            offset_of!(CuvidDecodeCreateInfo, enable_histogram),
            88,
        ),
        (
            "enable_decode_features",
            offset_of!(CuvidDecodeCreateInfo, enable_decode_features),
            92,
        ),
        (
            "reserved2",
            offset_of!(CuvidDecodeCreateInfo, reserved2),
            96,
        ),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(
            actual, expected,
            "{name} sits where CUVIDDECODECREATEINFO keeps it"
        );
    }
    assert_eq!(
        size_of::<CuvidDecodeCreateInfo>(),
        112,
        "the create info is as big as CUVIDDECODECREATEINFO"
    );
}
