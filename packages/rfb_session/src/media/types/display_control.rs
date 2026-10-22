use std::sync::Arc;

use ard_media::VirtualDisplay;

use super::MediaPeer;

pub(crate) struct DisplayControl {
    pub(crate) peer: Arc<dyn MediaPeer>,
    pub(crate) display: VirtualDisplay,
    pub(crate) scale: u32,
}
