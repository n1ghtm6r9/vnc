use std::{
    io::{ErrorKind, Read},
    net::TcpStream,
};

pub fn drain_until_closed(stream: &mut TcpStream) -> bool {
    let mut sink = [0u8; 4096];
    loop {
        match stream.read(&mut sink) {
            Ok(0) => return true,
            Ok(_) => {}
            Err(error) => {
                return !matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock);
            }
        }
    }
}
