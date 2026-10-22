use std::time::Instant;

use crate::{
    MediaChannel, MediaFailure, MediaState,
    test_support::{
        FIRST_DON, FakeDecoder, NAL_IDR, TestServer, answered_session, deliver, fake_unit, states,
    },
};

#[test]
fn test_a_decoder_error_ends_the_session() {
    let start = Instant::now();
    let mut session = answered_session(
        FakeDecoder {
            broken: true,
            ..FakeDecoder::default()
        },
        start,
    );
    let mut server = TestServer::new();

    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]),
        start,
    );

    let failures = states(&session.effects.take().events);
    assert!(
        matches!(failures[..], [MediaState::Failed(MediaFailure::Decoder(_))]),
        "a decoder error ends High Performance, got {failures:?}"
    );
}
