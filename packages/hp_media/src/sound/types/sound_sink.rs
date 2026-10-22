use hp_audio::SoundPacket;

pub(crate) trait SoundSink: Send {
    fn play(&mut self, packet: SoundPacket);
}
