use std::net::TcpStream;

use rfb::AppleServerInit;

use crate::test_support::{
    HIGH_PERFORMANCE_CAPABILITIES, MAC_SERVER_FLAGS, serve_apple_login, serve_apple_server_init,
    serve_version, server_init, test_credentials,
};

pub fn serve_high_performance_mac(stream: &mut TcpStream, width: u16, height: u16) {
    serve_version(stream);
    serve_apple_login(stream, &test_credentials());
    serve_apple_server_init(
        stream,
        &AppleServerInit {
            init: server_init(width, height),
            flags: MAC_SERVER_FLAGS,
            capabilities: HIGH_PERFORMANCE_CAPABILITIES,
        },
    );
}
