use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn serve_version(stream: &mut TcpStream) -> [u8; 12] {
    stream
        .write_all(b"RFB 003.889\n")
        .expect("the fake server sends the version of the Mac");
    let mut answer = [0u8; 12];
    stream
        .read_exact(&mut answer)
        .expect("the client answers with its version");
    answer
}
