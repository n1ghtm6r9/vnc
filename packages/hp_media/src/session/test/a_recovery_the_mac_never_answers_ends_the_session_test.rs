use std::time::{Duration, Instant};

use crate::{
    ANSWER_TIMEOUT, MediaFailure, MediaState, PICTURE_STALL_LIMIT,
    test_support::{states, streaming_session},
};

#[test]
fn test_a_recovery_the_mac_never_answers_ends_the_session() {
    let start = Instant::now();
    let (mut session, _server) = streaming_session(start);
    let frozen = start + PICTURE_STALL_LIMIT + Duration::from_millis(1);
    session.tick(frozen);
    session.effects.take();

    session.tick(frozen + ANSWER_TIMEOUT);

    assert_eq!(
        states(&session.effects.take().events),
        [MediaState::Failed(MediaFailure::MediaStopped)],
        "a fresh offer the Mac does not answer ends High Performance"
    );
}
