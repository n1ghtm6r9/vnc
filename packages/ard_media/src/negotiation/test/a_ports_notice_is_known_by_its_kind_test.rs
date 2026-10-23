use super::fixtures::{answer_payload, answer_stream, audio_blob, video_blob, video_fields};
use crate::is_ports_notice;

#[test]
fn test_a_ports_notice_is_known_by_its_kind() {
    let short_notice = [
        0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x17, 0x0d, 0x00, 0x00, 0x00,
        0x01,
    ];
    let answer = answer_payload(
        &answer_stream(&audio_blob(1)),
        &answer_stream(&video_blob(&video_fields(2))),
    );

    assert!(
        is_ports_notice(&short_notice),
        "a ports notice shorter than the usual one is still a ports notice"
    );
    assert!(!is_ports_notice(&answer), "an answer is not a ports notice");
    assert!(!is_ports_notice(&[0]), "a byte is not a ports notice");
}
