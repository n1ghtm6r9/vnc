use std::time::Instant;

use crate::{
    MediaChannel, MediaFailure, MediaState,
    test_support::{
        FIRST_DON, FakeDecoder, NAL_IDR, TestServer, answered_session, deliver, fake_unit, states,
        stripe_images,
    },
};

#[test]
fn test_video_in_another_format_ends_the_session() {
    let start = Instant::now();
    let mut session = answered_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();
    server.payload_type = 96;

    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]),
        start,
    );
    let effects = session.effects.take();

    assert!(
        stripe_images(&effects.events).is_empty(),
        "video that is not HEVC is not decoded"
    );
    assert_eq!(
        states(&effects.events),
        [MediaState::Failed(MediaFailure::UnsupportedVideo(96))],
        "it ends High Performance"
    );
}
