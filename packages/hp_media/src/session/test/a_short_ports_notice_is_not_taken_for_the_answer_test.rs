use std::time::Instant;

use crate::{
    ANSWER_TIMEOUT, MediaFailure, MediaState,
    test_support::{FakeDecoder, message1_payload, states, test_session},
};

#[test]
fn test_a_short_ports_notice_is_not_taken_for_the_answer() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);
    session.handle_message1(&message1_payload(), start);
    let short_notice = [
        0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x17, 0x0d, 0x00, 0x00, 0x00,
        0x01,
    ];

    session.handle_answer(&short_notice, start);
    session.tick(start + ANSWER_TIMEOUT);

    assert_eq!(
        states(&session.effects.take().events),
        [MediaState::Failed(MediaFailure::NoAnswer)],
        "a ports notice shorter than the usual one leaves the offer unanswered instead of failing as a bad answer"
    );
}
