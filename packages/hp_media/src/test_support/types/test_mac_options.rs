use std::net::{Ipv4Addr, UdpSocket};

use super::TestMac;
use crate::{MediaOptions, MediaPorts};

impl TestMac {
    pub fn options(&self) -> MediaOptions {
        let port = |socket: &UdpSocket| {
            socket
                .local_addr()
                .expect("a bound socket has an address")
                .port()
        };
        MediaOptions {
            server: Ipv4Addr::LOCALHOST.into(),
            ports: MediaPorts {
                local: [0, 0],
                remote: [port(&self.sockets[0]), port(&self.sockets[1])],
            },
            play_sound: false,
        }
    }
}
