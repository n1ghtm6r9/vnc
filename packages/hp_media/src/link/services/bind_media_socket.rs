use std::{
    io,
    net::{IpAddr, SocketAddr, UdpSocket},
};

use log::warn;
use socket2::{Domain, Protocol, Socket, Type};

use super::local_address;
use crate::{MEDIA_RECEIVE_BUFFER, RECEIVE_TIMEOUT};

pub(crate) fn bind_media_socket(server: IpAddr, port: u16) -> io::Result<UdpSocket> {
    let address = SocketAddr::new(local_address(server), port);
    let socket = Socket::new(
        Domain::for_address(address),
        Type::DGRAM,
        Some(Protocol::UDP),
    )?;
    if let Err(error) = socket.set_recv_buffer_size(MEDIA_RECEIVE_BUFFER) {
        warn!("[MEDIA] the receive buffer of port {port} keeps its default size: {error}");
    }
    socket.bind(&address.into())?;
    let socket = UdpSocket::from(socket);
    socket.set_read_timeout(Some(RECEIVE_TIMEOUT))?;
    Ok(socket)
}
