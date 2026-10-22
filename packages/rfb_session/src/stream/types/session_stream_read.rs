use std::{
    io::{self, ErrorKind, Read},
    time::Instant,
};

use super::SessionStream;

impl Read for SessionStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.control.is_stopped() {
                return Err(io::Error::new(
                    ErrorKind::ConnectionAborted,
                    "the session was stopped",
                ));
            }
            if self
                .deadline
                .is_some_and(|deadline| Instant::now() >= deadline)
            {
                return Err(io::Error::new(
                    ErrorKind::TimedOut,
                    "the server did not answer in time",
                ));
            }
            match self.socket.read(buf) {
                Err(error)
                    if matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) => {}
                result => return result,
            }
        }
    }
}
