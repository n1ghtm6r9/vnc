use hp_audio::SoundPacket;

#[derive(Debug, PartialEq, Eq)]
pub enum SoundNotice {
    Opened,
    Played(SoundPacket),
    Closed,
}
