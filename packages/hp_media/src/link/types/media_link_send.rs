use std::io;

use super::MediaLink;
use crate::Datagram;

impl MediaLink {
    pub(crate) fn send(&self, datagram: &Datagram) -> io::Result<()> {
        let index = datagram.channel.index();
        self.sockets[index]
            .send_to(&datagram.bytes, self.remote[index])
            .map(drop)
    }
}
