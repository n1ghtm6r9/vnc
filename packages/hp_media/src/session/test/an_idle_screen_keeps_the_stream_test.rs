use std::time::{Duration, Instant};

use crate::test_support::{offer_count, sound_until, states, streaming_session};

#[test]
fn test_an_idle_screen_keeps_the_stream() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);

    sound_until(
        &mut session,
        &mut server,
        start,
        start + Duration::from_secs(10),
    );
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        0,
        "a screen with nothing new to show is not asked for again"
    );
    assert!(
        states(&effects.events).is_empty(),
        "High Performance goes on while the screen is still"
    );
    assert!(!session.is_finished(), "the session goes on");
}
