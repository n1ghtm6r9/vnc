use std::{
    io::BufWriter,
    net::TcpStream,
    sync::{Arc, atomic::AtomicBool, mpsc::Receiver},
    time::Instant,
};

use rfb::Rect;
use rfb_auth::AppleSessionKey;

use super::Outgoing;
use crate::DisplayControl;

pub(crate) struct SessionWriter {
    pub(super) output: BufWriter<TcpStream>,
    pub(super) queue: Receiver<Outgoing>,
    pub(super) screen: Rect,
    pub(super) pointer_buttons: u8,
    pub(super) apple_key: Option<AppleSessionKey>,
    pub(super) started: Instant,
    pub(super) clipboard_session: u32,
    pub(super) clipboard_text: Option<String>,
    pub(super) streaming: Option<Arc<AtomicBool>>,
    pub(super) display: Option<DisplayControl>,
}
