use std::{io, net::TcpStream};

use socket2::{SockRef, TcpKeepalive};

use crate::{KEEPALIVE_IDLE, KEEPALIVE_INTERVAL};

pub(crate) fn enable_keepalive(socket: &TcpStream) -> io::Result<()> {
    let keepalive = TcpKeepalive::new()
        .with_time(KEEPALIVE_IDLE)
        .with_interval(KEEPALIVE_INTERVAL);
    SockRef::from(socket).set_tcp_keepalive(&keepalive)
}
