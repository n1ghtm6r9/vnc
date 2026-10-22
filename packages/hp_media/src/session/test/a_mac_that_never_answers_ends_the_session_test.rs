use std::time::{Duration, Instant};

use crate::{
    ANSWER_TIMEOUT, MediaFailure, MediaState,
    test_support::{FakeDecoder, message1_payload, states, test_session},
};

#[test]
fn test_a_mac_that_never_answers_ends_the_session() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);
    session.handle_message1(&message1_payload(), start);

    session.tick(start + ANSWER_TIMEOUT - Duration::from_millis(1));
    assert!(!session.is_finished(), "the Mac has five seconds to answer");

    session.tick(start + ANSWER_TIMEOUT);
    let failures = states(&session.effects.take().events);
    assert_eq!(
        failures,
        [MediaState::Failed(MediaFailure::NoAnswer)],
        "an unanswered offer ends High Performance"
    );
}
