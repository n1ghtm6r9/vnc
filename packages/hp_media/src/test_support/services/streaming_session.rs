use std::time::Instant;

use super::{answered_session, deliver, fake_unit};
use crate::{
    MediaChannel, MediaSession,
    test_support::{FIRST_DON, FakeDecoder, NAL_IDR, TestServer},
};

pub fn streaming_session(start: Instant) -> (MediaSession<FakeDecoder>, TestServer) {
    let mut session = answered_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();
    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 1)]),
        start,
    );
    session.effects.take();
    (session, server)
}
