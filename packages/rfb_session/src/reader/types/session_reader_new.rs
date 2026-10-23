use std::{io::BufReader, sync::mpsc::Sender, time::Instant};

use rfb::{ENCODING_RAW, PixelFormat, Rect};
use rfb_decode::{DecoderState, Framebuffer, PixelConverter};

use super::SessionReader;
use crate::{CountingReader, Outgoing, READ_BUFFER_SIZE, SessionStream, StatsMeter};

impl SessionReader {
    pub(crate) fn new(
        stream: SessionStream,
        screen: Rect,
        requests: Sender<Outgoing>,
        apple_remote_desktop: bool,
    ) -> SessionReader {
        SessionReader {
            input: BufReader::with_capacity(READ_BUFFER_SIZE, CountingReader::new(stream)),
            framebuffer: Framebuffer::new(screen.width, screen.height),
            converter: PixelConverter::new(&PixelFormat::rgba32()),
            decoder: DecoderState::new(),
            requests,
            meter: StatsMeter::new(Instant::now()),
            last_encoding: ENCODING_RAW,
            apple_remote_desktop,
            media: None,
            cursor_visible: None,
            input_source: None,
        }
    }
}
