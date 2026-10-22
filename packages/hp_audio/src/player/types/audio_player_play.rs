use super::AudioPlayer;
use crate::SoundPacket;

impl AudioPlayer {
    pub fn play(&self, packet: SoundPacket) {
        let _ = self.packets.send(packet);
    }
}
