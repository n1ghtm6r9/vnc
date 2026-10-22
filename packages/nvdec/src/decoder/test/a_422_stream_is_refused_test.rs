use crate::{CuvidVideoFormat, NvdecError, StreamFormat, test_support::test_video_format};

#[test]
fn test_a_422_stream_is_refused() {
    let video = CuvidVideoFormat {
        chroma_format: 2,
        ..test_video_format()
    };

    let refused = StreamFormat::read(&video);

    assert!(
        matches!(&refused, Err(NvdecError::Unsupported(reason)) if reason == "chroma format 2"),
        "a 4:2:2 stream is refused, got {refused:?}"
    );
}
