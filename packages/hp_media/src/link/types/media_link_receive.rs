use std::io::{self, ErrorKind};

use super::MediaLink;
use crate::MediaChannel;

impl MediaLink {
    pub(crate) fn receive(
        &self,
        channel: MediaChannel,
        buffer: &mut [u8],
    ) -> io::Result<Option<usize>> {
        let index = channel.index();
        match self.sockets[index].recv_from(buffer) {
            Ok((size, from)) => Ok((from.ip() == self.remote[index].ip()).then_some(size)),
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::WouldBlock
                        | ErrorKind::TimedOut
                        | ErrorKind::ConnectionReset
                        | ErrorKind::Interrupted
                ) =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
}
