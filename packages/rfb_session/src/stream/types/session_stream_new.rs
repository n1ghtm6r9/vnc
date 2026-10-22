use std::{
    io,
    net::TcpStream,
    sync::Arc,
    time::{Duration, Instant},
};

use super::SessionStream;
use crate::{POLL_INTERVAL, SessionControl};

impl SessionStream {
    pub(crate) fn new(
        socket: TcpStream,
        control: Arc<SessionControl>,
        answer_within: Duration,
    ) -> io::Result<SessionStream> {
        socket.set_read_timeout(Some(POLL_INTERVAL))?;
        Ok(SessionStream {
            socket,
            control,
            deadline: Instant::now().checked_add(answer_within),
        })
    }
}
