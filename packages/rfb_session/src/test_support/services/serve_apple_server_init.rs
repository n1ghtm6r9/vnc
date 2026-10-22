use std::net::TcpStream;

use rfb::{APPLE_CLIENT_INIT, AppleServerInit, read_u8};

pub fn serve_apple_server_init(stream: &mut TcpStream, apple_init: &AppleServerInit) {
    let client_init = read_u8(stream).expect("the client sends ClientInit");
    assert_eq!(
        client_init, APPLE_CLIENT_INIT,
        "the client asks for the extended ServerInit of the Mac"
    );
    apple_init
        .write(stream)
        .expect("the fake server sends the extended ServerInit");
}
