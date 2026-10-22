use std::time::{Duration, Instant};

use crate::{
    MediaChannel, MediaState, PICTURE_STALL_LIMIT,
    test_support::{
        FIRST_DON, NAL_IDR, NAL_TRAIL, TEST_PICTURE_HEIGHT, TEST_PICTURE_WIDTH, TEST_STRIPES,
        deliver, fake_unit, offer_count, sound_until, states, streaming_session, stripe_images,
        test_answer,
    },
};

#[test]
fn test_a_frozen_picture_is_recovered_with_a_fresh_offer() {
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
    let effects = session.effects.take();
    assert_eq!(
        offer_count(&effects.events),
        1,
        "a picture that never finishes is asked for again"
    );
    assert!(
        states(&effects.events).is_empty(),
        "High Performance goes on while the Mac answers"
    );

    session.handle_answer(&test_answer(), frozen);
    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 4)]),
        frozen,
    );
    let effects = session.effects.take();

    assert_eq!(
        stripe_images(&effects.events).len(),
        1,
        "the fresh stream is shown"
    );
    assert_eq!(
        states(&effects.events),
        [MediaState::Streaming {
            width: TEST_PICTURE_WIDTH,
            height: TEST_PICTURE_HEIGHT,
            stripes: TEST_STRIPES,
        }],
        "the fresh stream reports its picture size"
    );
    assert!(!session.is_finished(), "the session goes on");
}
