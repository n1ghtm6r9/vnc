use std::net::TcpStream;

use rfb::AppleServerInit;

use crate::test_support::{serve_apple_server_init, server_init};

pub fn serve_apple_init(stream: &mut TcpStream, width: u16, height: u16, flags: u32) {
    serve_apple_server_init(
        stream,
        &AppleServerInit {
            init: server_init(width, height),
            flags,
            capabilities: [0; 16],
        },
    );
}
