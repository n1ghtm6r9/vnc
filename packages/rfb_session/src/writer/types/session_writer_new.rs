use std::{io::BufWriter, net::TcpStream, sync::mpsc::Receiver, time::Instant};

use rfb::Rect;
use rfb_auth::AppleSessionKey;

use super::{Outgoing, SessionWriter};

impl SessionWriter {
    pub(crate) fn new(
        socket: TcpStream,
        queue: Receiver<Outgoing>,
        screen: Rect,
        apple_key: Option<AppleSessionKey>,
    ) -> SessionWriter {
        SessionWriter {
            output: BufWriter::new(socket),
            queue,
            screen,
            pointer_buttons: 0,
            apple_key,
            started: Instant::now(),
            clipboard_session: 0,
            clipboard_text: None,
            streaming: None,
            display: None,
        }
    }
}
