use ard_media::generate_secrets;
use nvdec::HevcDecoder;

use super::launch_media;
use crate::{MediaEvent, MediaFactories, MediaHandle, MediaOptions, MediaStartError, start_player};

pub fn start_media(
    options: MediaOptions,
    on_event: impl FnMut(MediaEvent) + Send + 'static,
) -> Result<MediaHandle, MediaStartError> {
    launch_media(
        options,
        on_event,
        MediaFactories {
            new_decoder: HevcDecoder::new,
            new_secrets: generate_secrets,
            new_sound: Box::new(start_player),
        },
    )
}
