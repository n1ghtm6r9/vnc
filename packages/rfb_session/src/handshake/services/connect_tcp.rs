use std::{
    io,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use log::warn;

use super::enable_keepalive;

pub(crate) fn connect_tcp(host: &str, port: u16, timeout: Duration) -> io::Result<TcpStream> {
    let mut last_error = io::Error::new(
        io::ErrorKind::NotFound,
        format!("the host name {host:?} resolves to no address"),
    );
    for address in (host, port).to_socket_addrs()? {
        match TcpStream::connect_timeout(&address, timeout) {
            Ok(socket) => {
                socket.set_nodelay(true)?;
                if let Err(error) = enable_keepalive(&socket) {
                    warn!("[SESSION] a lost link will be noticed only on the next input: {error}");
                }
                return Ok(socket);
            }
            Err(error) => last_error = error,
        }
    }
    Err(last_error)
}
