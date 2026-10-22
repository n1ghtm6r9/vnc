use std::net::UdpSocket;

use super::TestServer;

pub struct TestMac {
    pub sockets: [UdpSocket; 2],
    pub server: TestServer,
}
