use std::time::{Duration, Instant};

use crate::{
    MediaChannel, MediaState, PICTURE_STALL_LIMIT, RECOVERY_LIMIT,
    test_support::{
        FIRST_DON, NAL_IDR, deliver, fake_unit, offer_count, states, streaming_session, test_answer,
    },
};

#[test]
fn test_recovery_stops_after_three_attempts_in_thirty_seconds() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);
    let mut now = start;

    for attempt in 1..=RECOVERY_LIMIT {
        now += PICTURE_STALL_LIMIT + Duration::from_millis(1);
        session.tick(now);
        assert_eq!(
            offer_count(&session.effects.take().events),
            1,
            "glitch {attempt} asks the Mac again"
        );
        session.handle_answer(&test_answer(), now);
        deliver(
            &mut session,
            MediaChannel::Video,
            server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]),
            now,
        );
        session.effects.take();
    }
    now += PICTURE_STALL_LIMIT + Duration::from_millis(1);
    session.tick(now);
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        0,
        "a fourth glitch within thirty seconds is not retried"
    );
    assert!(
        matches!(states(&effects.events)[..], [MediaState::Failed(_)]),
        "it ends High Performance, got {:?}",
        states(&effects.events)
    );
}
