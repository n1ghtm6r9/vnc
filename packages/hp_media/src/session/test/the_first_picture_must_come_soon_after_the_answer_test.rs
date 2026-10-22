use std::time::{Duration, Instant};

use crate::{
    FIRST_PICTURE_TIMEOUT, MediaFailure, MediaState,
    test_support::{FakeDecoder, answered_session, states},
};

#[test]
fn test_the_first_picture_must_come_soon_after_the_answer() {
    let start = Instant::now();
    let mut session = answered_session(FakeDecoder::default(), start);

    session.tick(start + FIRST_PICTURE_TIMEOUT - Duration::from_millis(1));
    assert!(
        !session.is_finished(),
        "the first picture has five seconds to arrive"
    );

    session.tick(start + FIRST_PICTURE_TIMEOUT);
    assert_eq!(
        states(&session.effects.take().events),
        [MediaState::Failed(MediaFailure::NoPicture)],
        "no picture in five seconds ends High Performance"
    );
}
