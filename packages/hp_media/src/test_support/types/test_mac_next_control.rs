use std::time::Instant;

use super::{Control, TestMac};
use crate::{Datagram, MediaChannel, test_support::EVENT_TIMEOUT};

impl TestMac {
    pub fn next_control(
        &self,
        channel: MediaChannel,
        wanted: impl Fn(&Control) -> bool,
    ) -> Control {
        let mut buffer = [0; 2048];
        let deadline = Instant::now() + EVENT_TIMEOUT;
        loop {
            assert!(
                Instant::now() < deadline,
                "the awaited feedback reaches the Mac in time"
            );
            let (size, _) = self.sockets[channel.index()]
                .recv_from(&mut buffer)
                .expect("feedback reaches the Mac in time");
            let control = self.server.open(&Datagram {
                channel,
                bytes: buffer[..size].to_vec(),
            });
            if wanted(&control) {
                return control;
            }
        }
    }
}
