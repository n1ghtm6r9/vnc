use std::{io::Read, net::TcpStream};

pub fn read_framed_message(stream: &mut TcpStream) -> Vec<u8> {
    let mut message = vec![0u8; 4];
    stream
        .read_exact(&mut message)
        .expect("the client sends the header of a framed Apple message");
    let length = usize::from(u16::from_be_bytes([message[2], message[3]]));
    message.resize(4 + length, 0);
    stream
        .read_exact(&mut message[4..])
        .expect("the client sends the body of a framed Apple message");
    message
}
