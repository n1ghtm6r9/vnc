use std::{io, net::TcpStream};

use super::SessionStream;

impl SessionStream {
    pub(crate) fn try_clone_socket(&self) -> io::Result<TcpStream> {
        self.socket.try_clone()
    }
}
