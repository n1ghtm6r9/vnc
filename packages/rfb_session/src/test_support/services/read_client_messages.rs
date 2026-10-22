use std::net::TcpStream;

use rfb::ClientMessage;

pub fn read_client_messages(stream: &mut TcpStream, count: usize) -> Vec<ClientMessage> {
    (0..count)
        .map(|_| ClientMessage::read(stream).expect("the client sends a readable message"))
        .collect()
}
