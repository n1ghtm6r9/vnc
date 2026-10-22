use std::sync::{Arc, atomic::AtomicBool, mpsc::Sender};

use super::MediaInput;

#[derive(Debug)]
pub struct MediaHandle {
    pub(crate) inputs: Sender<MediaInput>,
    pub(crate) stop: Arc<AtomicBool>,
    pub(crate) local_ports: [u16; 2],
}
