use super::fixtures::{answer_payload, answer_stream, audio_blob, video_blob, video_fields};
use crate::{MediaAnswer, VideoFormat, parse_answer};

#[test]
fn test_an_answer_gives_the_server_sources_and_picture_size() {
    let payload = answer_payload(
        &answer_stream(&audio_blob(0xa656_a732)),
        &answer_stream(&video_blob(&video_fields(0x9d81_196f))),
    );

    assert_eq!(
        parse_answer(&payload).expect("a well-formed answer reads"),
        MediaAnswer {
            version: 2,
            audio_ssrc: 0xa656_a732,
            video: VideoFormat {
                ssrc: 0x9d81_196f,
                width: 3840,
                height: 2160,
                stripes: 4,
            },
        },
        "the answer names both server sources, with the top bit set, and the picture in four stripes"
    );
}
