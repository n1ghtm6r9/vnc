use super::fixtures::{answer_payload, answer_stream, audio_blob, video_blob, video_fields};
use crate::{ProtoField, parse_answer};

#[test]
fn test_a_bad_answer_is_refused() {
    let audio = answer_stream(&audio_blob(1));
    let video = answer_stream(&video_blob(&video_fields(2)));
    let good = answer_payload(&audio, &video);
    let no_size = answer_stream(&video_blob(&video_fields(2)[..1]));
    let no_stripes = answer_stream(&video_blob(&[
        ProtoField::varint(1, 2),
        ProtoField::varint(4, 3840),
        ProtoField::varint(5, 2160),
        ProtoField::varint(6, 0),
    ]));
    let garbage: Vec<u8> = (0..64u8).map(|index| index.wrapping_mul(7)).collect();

    assert!(parse_answer(&good).is_ok(), "the unchanged answer reads");
    assert!(
        parse_answer(&good[..good.len() - 20]).is_err(),
        "a cut answer is refused"
    );
    assert!(
        parse_answer(&[0]).is_err(),
        "an answer without its length is refused"
    );
    assert!(
        parse_answer(&answer_payload(&[], &video)).is_err(),
        "an answer without audio is refused"
    );
    assert!(
        parse_answer(&answer_payload(&audio, &no_size)).is_err(),
        "an answer without a picture size is refused"
    );
    assert!(
        parse_answer(&answer_payload(&audio, &no_stripes)).is_err(),
        "an answer of zero stripes is refused"
    );
    assert!(
        parse_answer(&answer_payload(&audio, &garbage)).is_err(),
        "an answer of noise is refused"
    );
}
