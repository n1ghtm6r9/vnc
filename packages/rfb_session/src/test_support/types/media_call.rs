#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaCall {
    Message1(Vec<u8>),
    Answer(Vec<u8>),
    Resize(Vec<u8>),
    PlaySound(bool),
}
