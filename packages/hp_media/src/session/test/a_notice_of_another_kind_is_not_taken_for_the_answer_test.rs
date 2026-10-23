use std::time::Instant;

use crate::{
    ANSWER_TIMEOUT, MediaFailure, MediaState,
    test_support::{FakeDecoder, message1_payload, states, test_session},
};

#[test]
fn test_a_notice_of_another_kind_is_not_taken_for_the_answer() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);
    session.handle_message1(&message1_payload(), start);
    let notice_of_another_kind = [
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00,
    ];

    session.handle_answer(&notice_of_another_kind, start);
    session.tick(start + ANSWER_TIMEOUT);

    assert_eq!(
        states(&session.effects.take().events),
        [MediaState::Failed(MediaFailure::NoAnswer)],
        "a notice of a kind the client does not know leaves the offer unanswered instead of failing as a bad answer"
    );
}
