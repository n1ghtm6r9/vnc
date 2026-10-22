use std::sync::mpsc::Sender;

use crate::SoundPacket;

#[derive(Debug)]
pub struct AudioPlayer {
    pub(crate) packets: Sender<SoundPacket>,
}
