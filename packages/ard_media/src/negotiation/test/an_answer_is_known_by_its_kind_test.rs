use super::fixtures::{answer_payload, answer_stream, audio_blob, video_blob, video_fields};
use crate::is_answer;

#[test]
fn test_an_answer_is_known_by_its_kind() {
    let answer = answer_payload(
        &answer_stream(&audio_blob(1)),
        &answer_stream(&video_blob(&video_fields(2))),
    );
    let short_ports_notice = [
        0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x17, 0x0d, 0x00, 0x00, 0x00,
        0x01,
    ];
    let notice_of_another_kind = [
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00,
    ];

    assert!(is_answer(&answer), "an answer is known by its kind");
    assert!(
        !is_answer(&short_ports_notice),
        "a ports notice is not an answer"
    );
    assert!(
        !is_answer(&notice_of_another_kind),
        "a notice of another kind is not an answer"
    );
    assert!(!is_answer(&[0]), "a byte is not an answer");
}
