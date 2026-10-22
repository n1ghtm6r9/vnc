use crate::{CuvidVideoFormat, NvdecError, StreamFormat, test_support::test_video_format};

#[test]
fn test_a_10_bit_stream_is_refused() {
    let video = CuvidVideoFormat {
        bit_depth_luma_minus8: 2,
        bit_depth_chroma_minus8: 2,
        ..test_video_format()
    };

    let refused = StreamFormat::read(&video);

    assert!(
        matches!(&refused, Err(NvdecError::Unsupported(reason)) if reason == "10-bit video"),
        "a 10-bit stream is refused by name, got {refused:?}"
    );
}
