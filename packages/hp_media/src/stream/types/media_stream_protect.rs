use super::MediaStream;
use crate::{Datagram, MediaChannel};

impl MediaStream {
    pub(crate) fn protect(&mut self, channel: MediaChannel, plain: &[u8]) -> Option<Datagram> {
        let bytes = self.outgoing[channel.index()].protect_rtcp(plain)?;
        Some(Datagram { channel, bytes })
    }
}
