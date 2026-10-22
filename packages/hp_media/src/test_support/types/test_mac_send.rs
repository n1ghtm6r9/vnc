use std::net::Ipv4Addr;

use super::TestMac;
use crate::{MediaChannel, MediaHandle};

impl TestMac {
    pub fn send(&self, channel: MediaChannel, datagrams: &[Vec<u8>], handle: &MediaHandle) {
        let port = handle.local_ports()[channel.index()];
        for datagram in datagrams {
            self.sockets[channel.index()]
                .send_to(datagram, (Ipv4Addr::LOCALHOST, port))
                .expect("a datagram goes out on loopback");
        }
    }
}
