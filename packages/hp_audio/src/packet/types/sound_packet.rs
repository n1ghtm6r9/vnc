#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundPacket {
    pub ssrc: u32,
    pub seq: u16,
    pub payload: Vec<u8>,
}
