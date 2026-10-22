use crate::MediaChannel;

pub(crate) enum MediaInput {
    Message1(Vec<u8>),
    Answer(Vec<u8>),
    Resize(Vec<u8>),
    PlaySound(bool),
    Datagram(MediaChannel, Vec<u8>),
    LinkFailed(String),
    Stop,
}
