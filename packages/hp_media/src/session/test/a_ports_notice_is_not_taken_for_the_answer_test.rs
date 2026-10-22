use std::time::Instant;

use crate::{
    ANSWER_TIMEOUT, MediaFailure, MediaState,
    test_support::{FakeDecoder, message1_payload, ports_notice_payload, states, test_session},
};

#[test]
fn test_a_ports_notice_is_not_taken_for_the_answer() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);
    session.handle_message1(&message1_payload(), start);

    session.handle_answer(&ports_notice_payload(), start);
    session.tick(start + ANSWER_TIMEOUT);

    let failures = states(&session.effects.take().events);
    assert_eq!(
        failures,
        [MediaState::Failed(MediaFailure::NoAnswer)],
        "the Mac's ports notice leaves the offer unanswered"
    );
}
