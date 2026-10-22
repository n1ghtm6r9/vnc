use hp_audio::SoundPacket;

use super::{FakeSoundSink, SoundNotice};
use crate::SoundSink;

impl SoundSink for FakeSoundSink {
    fn play(&mut self, packet: SoundPacket) {
        let _ = self.notices.send(SoundNotice::Played(packet));
    }
}
