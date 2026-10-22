use std::time::{Duration, Instant};

use crate::{
    MediaChannel, PICTURE_STALL_LIMIT,
    test_support::{
        FIRST_DON, NAL_IDR, NAL_TRAIL, SOUND_INTERVAL, deliver, fake_unit, offer_count,
        sound_until, streaming_session, test_answer,
    },
};

#[test]
fn test_a_fresh_stream_that_shows_nothing_is_asked_for_again() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);
    let mut unfinished = server.picture(
        FIRST_DON + 1,
        0,
        &[&fake_unit(NAL_TRAIL, 2), &fake_unit(NAL_TRAIL, 3)],
    );
    unfinished.truncate(1);
    deliver(&mut session, MediaChannel::Video, unfinished, start);
    let frozen = start + PICTURE_STALL_LIMIT + Duration::from_millis(1);
    sound_until(&mut session, &mut server, start, frozen);
    session.effects.take();
    session.handle_answer(&test_answer(), frozen);
    session.decoder.silent = true;
    let given_up = frozen + PICTURE_STALL_LIMIT + SOUND_INTERVAL;

    let mut now = frozen;
    let mut don = FIRST_DON;
    while now < given_up {
        let kind = if don == FIRST_DON { NAL_IDR } else { NAL_TRAIL };
        deliver(
            &mut session,
            MediaChannel::Video,
            server.picture(don, 0, &[&fake_unit(kind, 4)]),
            now,
        );
        sound_until(&mut session, &mut server, now, now + SOUND_INTERVAL);
        now += SOUND_INTERVAL;
        don += 1;
    }
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        1,
        "a fresh stream whose pictures never show is asked for again"
    );
}
