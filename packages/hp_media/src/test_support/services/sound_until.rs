use std::time::Instant;

use super::deliver;
use crate::{
    MediaChannel, MediaSession,
    test_support::{FakeDecoder, SOUND_INTERVAL, TestServer},
};

pub fn sound_until(
    session: &mut MediaSession<FakeDecoder>,
    server: &mut TestServer,
    from: Instant,
    until: Instant,
) {
    let mut now = from;
    while now < until {
        now = (now + SOUND_INTERVAL).min(until);
        deliver(session, MediaChannel::Audio, vec![server.audio()], now);
        session.tick(now);
    }
}
