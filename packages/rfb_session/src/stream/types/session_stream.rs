use std::{net::TcpStream, sync::Arc, time::Instant};

use crate::SessionControl;

pub(crate) struct SessionStream {
    pub(super) socket: TcpStream,
    pub(super) control: Arc<SessionControl>,
    pub(super) deadline: Option<Instant>,
}
