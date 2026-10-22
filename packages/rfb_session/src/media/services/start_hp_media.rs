use hp_media::{MediaOptions, MediaStartError, start_media};

use crate::{MediaCallback, MediaPeer};

pub(crate) fn start_hp_media(
    options: MediaOptions,
    on_event: MediaCallback,
) -> Result<Box<dyn MediaPeer>, MediaStartError> {
    Ok(Box::new(start_media(options, on_event)?))
}
