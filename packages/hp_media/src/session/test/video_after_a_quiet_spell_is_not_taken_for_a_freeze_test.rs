use std::time::{Duration, Instant};

use crate::{
    MediaChannel,
    test_support::{
        FIRST_DON, NAL_TRAIL, deliver, fake_unit, offer_count, sound_until, streaming_session,
        stripe_images,
    },
};

#[test]
fn test_video_after_a_quiet_spell_is_not_taken_for_a_freeze() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);
    let quiet_end = start + Duration::from_secs(10);
    sound_until(&mut session, &mut server, start, quiet_end);
    session.effects.take();

    let mut first_half = server.picture(
        FIRST_DON + 1,
        0,
        &[&fake_unit(NAL_TRAIL, 2), &fake_unit(NAL_TRAIL, 3)],
    );
    let second_half = first_half.split_off(1);
    deliver(&mut session, MediaChannel::Video, first_half, quiet_end);
    session.tick(quiet_end + Duration::from_millis(5));
    deliver(
        &mut session,
        MediaChannel::Video,
        second_half,
        quiet_end + Duration::from_millis(10),
    );
    let effects = session.effects.take();

    assert_eq!(
        offer_count(&effects.events),
        0,
        "the first packets after a quiet spell are not a freeze"
    );
    assert_eq!(
        stripe_images(&effects.events).len(),
        1,
        "the new picture is shown"
    );
}
