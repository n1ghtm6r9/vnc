use hp_audio::SoundPacket;

use super::Datagram;
use crate::MediaEvent;

#[derive(Debug, Default)]
pub(crate) struct MediaEffects {
    pub(crate) events: Vec<MediaEvent>,
    pub(crate) datagrams: Vec<Datagram>,
    pub(crate) sounds: Vec<SoundPacket>,
}
