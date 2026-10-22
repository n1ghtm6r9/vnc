use std::io;

use super::MediaLink;

impl MediaLink {
    pub(crate) fn local_ports(&self) -> io::Result<[u16; 2]> {
        let [audio, video] = &self.sockets;
        Ok([audio.local_addr()?.port(), video.local_addr()?.port()])
    }
}
