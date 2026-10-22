use std::net::TcpStream;

use rfb::read_client_init;

use crate::test_support::server_init;

pub fn serve_init(stream: &mut TcpStream, width: u16, height: u16) {
    read_client_init(stream).expect("the client sends ClientInit");
    server_init(width, height)
        .write(stream)
        .expect("the fake server sends ServerInit");
}
