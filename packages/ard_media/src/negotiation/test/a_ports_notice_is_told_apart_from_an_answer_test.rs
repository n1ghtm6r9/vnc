use super::fixtures::{answer_payload, answer_stream, audio_blob, video_blob, video_fields};
use crate::{StreamPorts, parse_answer, parse_stream_ports};

#[test]
fn test_a_ports_notice_is_told_apart_from_an_answer() {
    let mut notice = vec![
        0x00, 0x24, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x17, 0x0c, 0x00, 0x00, 0x00,
        0x01, 0x17, 0x0d, 0x00, 0x00, 0x00, 0x01,
    ];
    notice.extend_from_slice(&[0; 16]);
    let answer = answer_payload(
        &answer_stream(&audio_blob(1)),
        &answer_stream(&video_blob(&video_fields(2))),
    );

    assert_eq!(
        parse_stream_ports(&notice),
        Some(StreamPorts {
            audio_port: 5900,
            video_port: 5901
        }),
        "the notice names the audio and video ports"
    );
    assert!(
        parse_answer(&notice).is_err(),
        "a notice is not taken for an answer"
    );
    assert_eq!(
        parse_stream_ports(&answer),
        None,
        "an answer is not taken for a notice"
    );
    assert_eq!(parse_stream_ports(&[0]), None, "a byte is not a notice");
    assert_eq!(
        parse_stream_ports(&notice[..12]),
        None,
        "a cut notice is not a notice"
    );
}
