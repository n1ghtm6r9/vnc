use std::net::TcpStream;

use rfb::ClientMessage;
use rfb_auth::AppleLogin;

use crate::test_support::{
    MAC_SERVER_FLAGS, read_client_messages, serve_apple_init, serve_apple_login, serve_version,
    test_credentials,
};

pub fn serve_apple_until_connected(
    stream: &mut TcpStream,
    width: u16,
    height: u16,
) -> (AppleLogin, Vec<ClientMessage>) {
    let answer = serve_version(stream);
    assert_eq!(
        &answer, b"RFB 003.889\n",
        "in Apple Remote Desktop mode the client answers with the version of the Mac"
    );
    let login = serve_apple_login(stream, &test_credentials());
    serve_apple_init(stream, width, height, MAC_SERVER_FLAGS);
    (login, read_client_messages(stream, 9))
}
