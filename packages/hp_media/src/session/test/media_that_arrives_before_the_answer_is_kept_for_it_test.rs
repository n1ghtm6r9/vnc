use std::time::Instant;

use crate::{
    MediaChannel, MediaState,
    test_support::{
        FIRST_DON, FakeDecoder, NAL_IDR, TestServer, deliver, fake_unit, message1_payload, states,
        stripe_images, test_answer, test_session,
    },
};

#[test]
fn test_media_that_arrives_before_the_answer_is_kept_for_it() {
    let start = Instant::now();
    let mut session = test_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();
    session.handle_message1(&message1_payload(), start);

    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 5)]),
        start,
    );
    assert!(
        stripe_images(&session.effects.take().events).is_empty(),
        "nothing is shown before the answer"
    );

    session.handle_answer(&test_answer(), start);
    let effects = session.effects.take();

    assert_eq!(
        stripe_images(&effects.events).len(),
        1,
        "the early picture is shown once the answer is in"
    );
    assert!(
        matches!(states(&effects.events)[..], [MediaState::Streaming { .. }]),
        "the early picture starts High Performance"
    );
}
