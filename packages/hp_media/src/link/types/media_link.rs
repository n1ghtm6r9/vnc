use std::net::{SocketAddr, UdpSocket};

pub(crate) struct MediaLink {
    pub(crate) sockets: [UdpSocket; 2],
    pub(crate) remote: [SocketAddr; 2],
}
