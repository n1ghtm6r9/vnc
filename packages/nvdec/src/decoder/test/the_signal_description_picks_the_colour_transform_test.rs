use crate::{
    ColourMatrix, ColourPrimaries, ColourTransform, CuvidVideoFormat, StreamFormat,
    VIDEO_FULL_RANGE_FLAG, test_support::test_video_format,
};

#[test]
fn test_the_signal_description_picks_the_colour_transform() {
    let full_range_601 = CuvidVideoFormat {
        video_signal_flags: VIDEO_FULL_RANGE_FLAG | 0x05,
        matrix_coefficients: 6,
        ..test_video_format()
    };
    let display_p3 = CuvidVideoFormat {
        colour_primaries: 12,
        transfer_characteristics: 13,
        ..test_video_format()
    };

    let full = StreamFormat::read(&full_range_601).expect("a full range stream is supported");
    let limited = StreamFormat::read(&test_video_format()).expect("the test format is supported");
    let wide = StreamFormat::read(&display_p3).expect("a Display P3 stream is supported");

    assert_eq!(
        full.colour,
        ColourTransform::new(ColourMatrix::Bt601, true, ColourPrimaries::Bt709),
        "the full range flag and matrix code 6 pick full range BT.601"
    );
    assert_eq!(
        limited.colour,
        ColourTransform::new(ColourMatrix::Bt709, false, ColourPrimaries::Bt709),
        "a clear flag and matrix code 1 pick limited range BT.709"
    );
    assert_eq!(
        wide.colour,
        ColourTransform::new(ColourMatrix::Bt709, false, ColourPrimaries::DisplayP3),
        "primaries code 12 maps the Display P3 light of the Mac into sRGB"
    );
}
