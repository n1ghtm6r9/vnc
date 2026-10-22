use std::net::{Ipv4Addr, UdpSocket};

use super::{TestMac, TestServer};
use crate::test_support::EVENT_TIMEOUT;

impl TestMac {
    pub fn new() -> TestMac {
        let socket = || {
            let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free loopback port");
            socket
                .set_read_timeout(Some(EVENT_TIMEOUT))
                .expect("the socket takes a read timeout");
            socket
        };
        TestMac {
            sockets: [socket(), socket()],
            server: TestServer::new(),
        }
    }
}
