use std::{io::BufReader, sync::mpsc::Sender};

use rfb_decode::{DecoderState, Framebuffer, PixelConverter};

use crate::{CountingReader, Outgoing, SessionMedia, SessionStream, StatsMeter};

pub(crate) struct SessionReader {
    pub(super) input: BufReader<CountingReader<SessionStream>>,
    pub(super) framebuffer: Framebuffer,
    pub(super) converter: PixelConverter,
    pub(super) decoder: DecoderState,
    pub(super) requests: Sender<Outgoing>,
    pub(super) meter: StatsMeter,
    pub(super) last_encoding: i32,
    pub(super) apple_remote_desktop: bool,
    pub(super) media: Option<SessionMedia>,
    pub(super) cursor_visible: Option<bool>,
}
