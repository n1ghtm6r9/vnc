use std::net::TcpStream;

use rfb::{SECURITY_APPLE_REMOTE_DESKTOP, read_u8, write_security_result, write_security_types};
use rfb_auth::{AppleLogin, Credentials, serve_apple_auth};

use crate::test_support::{MAC_SECURITY_TYPES, apple_server_options};

pub fn serve_apple_login(stream: &mut TcpStream, expected: &Credentials) -> AppleLogin {
    write_security_types(stream, &MAC_SECURITY_TYPES)
        .expect("the fake server offers the security types of the Mac");
    let choice = read_u8(stream).expect("the client picks a security type");
    assert_eq!(
        choice, SECURITY_APPLE_REMOTE_DESKTOP,
        "the client picks Apple authentication when it has a user name"
    );
    let login = serve_apple_auth(stream, apple_server_options())
        .expect("the client completes the Apple key exchange");
    let verdict = (&login.credentials != expected).then_some("authentication failed");
    write_security_result(stream, verdict).expect("the fake server sends the security result");
    login
}
