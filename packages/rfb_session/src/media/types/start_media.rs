use hp_media::{MediaOptions, MediaStartError};

use super::{MediaCallback, MediaPeer};

pub(crate) type StartMedia = Box<
    dyn FnMut(MediaOptions, MediaCallback) -> Result<Box<dyn MediaPeer>, MediaStartError> + Send,
>;
