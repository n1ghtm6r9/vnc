use std::time::Instant;

use crate::{
    MediaFailure, MediaState,
    test_support::{FakeDecoder, message1_payload, states, test_session},
};

#[test]
fn test_an_answer_that_cannot_be_read_ends_the_session() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);
    session.handle_message1(&message1_payload(), start);

    session.handle_answer(&[0, 4, 0, 2, 1, 2], start);

    let failures = states(&session.effects.take().events);
    assert!(
        matches!(
            failures[..],
            [MediaState::Failed(MediaFailure::BadAnswer(_))]
        ),
        "a broken answer ends High Performance, got {failures:?}"
    );
}
