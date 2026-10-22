use std::time::{Duration, Instant};

use crate::{
    MediaChannel, PICTURE_STALL_LIMIT, RECOVERY_LIMIT,
    test_support::{
        FIRST_DON, NAL_IDR, deliver, fake_unit, offer_count, streaming_session, test_answer,
    },
};

#[test]
fn test_resizing_spends_no_recovery_attempt() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);

    for resize in 1..=RECOVERY_LIMIT + 1 {
        session.resize(vec![0x1d], start);
        assert_eq!(
            offer_count(&session.effects.take().events),
            1,
            "resize {resize} asks the Mac for a fresh stream"
        );
        session.handle_answer(&test_answer(), start);
        deliver(
            &mut session,
            MediaChannel::Video,
            server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]),
            start,
        );
        session.effects.take();
    }
    session.tick(start + PICTURE_STALL_LIMIT + Duration::from_millis(1));
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        1,
        "after more resizes than the recovery limit a frozen picture is still asked for again"
    );
}
