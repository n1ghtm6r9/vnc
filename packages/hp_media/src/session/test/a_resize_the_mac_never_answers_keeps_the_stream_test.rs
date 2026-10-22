use std::time::{Duration, Instant};

use crate::{
    ANSWER_TIMEOUT,
    test_support::{offer_count, sound_until, states, streaming_session},
};

#[test]
fn test_a_resize_the_mac_never_answers_keeps_the_stream() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);
    session.resize(vec![0x1d], start);
    assert_eq!(
        offer_count(&session.effects.take().events),
        1,
        "the resize asks the Mac for a fresh stream"
    );

    sound_until(
        &mut session,
        &mut server,
        start,
        start + ANSWER_TIMEOUT + Duration::from_millis(1),
    );
    let effects = session.effects.take();

    assert!(
        states(&effects.events).is_empty(),
        "a display the Mac never answers does not end High Performance, got {:?}",
        states(&effects.events)
    );
    assert!(
        !session.is_finished(),
        "the session goes on with the stream it has"
    );
}
