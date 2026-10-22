use hp_audio::{AudioPlayer, SoundPacket};

use super::SoundSink;

impl SoundSink for AudioPlayer {
    fn play(&mut self, packet: SoundPacket) {
        AudioPlayer::play(self, packet);
    }
}
