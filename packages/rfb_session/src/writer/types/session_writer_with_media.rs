use std::sync::{Arc, atomic::AtomicBool};

use super::SessionWriter;

impl SessionWriter {
    pub(crate) fn with_media(mut self, streaming: Option<Arc<AtomicBool>>) -> SessionWriter {
        self.streaming = streaming;
        self
    }
}
