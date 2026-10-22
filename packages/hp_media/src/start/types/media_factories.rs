use ard_media::{MediaError, MediaSecrets};
use nvdec::NvdecError;

use crate::SoundFactory;

pub(crate) struct MediaFactories<D> {
    pub(crate) new_decoder: fn() -> Result<D, NvdecError>,
    pub(crate) new_secrets: fn() -> Result<MediaSecrets, MediaError>,
    pub(crate) new_sound: SoundFactory,
}
