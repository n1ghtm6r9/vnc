use std::net::TcpStream;

use rfb::ClientMessage;

use crate::test_support::{read_client_messages, read_framed_message, serve_high_performance_mac};

pub fn serve_high_performance_until_connected(
    stream: &mut TcpStream,
    width: u16,
    height: u16,
) -> (Vec<ClientMessage>, Vec<u8>) {
    serve_high_performance_mac(stream, width, height);
    let mut setup = read_client_messages(stream, 2);
    let virtual_display = read_framed_message(stream);
    setup.extend(read_client_messages(stream, 7));
    (setup, virtual_display)
}
