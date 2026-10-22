use std::time::Instant;

use crate::{MediaChannel, MediaSession, test_support::FakeDecoder};

pub fn deliver(
    session: &mut MediaSession<FakeDecoder>,
    channel: MediaChannel,
    datagrams: Vec<Vec<u8>>,
    now: Instant,
) {
    for mut datagram in datagrams {
        session.handle_datagram(channel, &mut datagram, now);
    }
}
