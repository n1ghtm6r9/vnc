use std::net::TcpStream;

use rfb::ClientMessage;
use rfb_auth::Credentials;

use crate::test_support::{
    read_client_messages, serve_apple_login, serve_init, serve_version, test_credentials,
};

pub fn serve_until_connected(
    stream: &mut TcpStream,
    width: u16,
    height: u16,
) -> (Credentials, Vec<ClientMessage>) {
    serve_version(stream);
    let login = serve_apple_login(stream, &test_credentials());
    serve_init(stream, width, height);
    (login.credentials, read_client_messages(stream, 3))
}
