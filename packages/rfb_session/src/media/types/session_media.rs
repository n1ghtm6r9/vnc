use std::sync::{Arc, atomic::AtomicBool};

use ard_media::VirtualDisplay;

use super::MediaPeer;

pub(crate) struct SessionMedia {
    pub(crate) peer: Arc<dyn MediaPeer>,
    pub(crate) streaming: Arc<AtomicBool>,
    pub(crate) display: VirtualDisplay,
}
