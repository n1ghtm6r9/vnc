#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtpPacket<'a> {
    pub ssrc: u32,
    pub seq: u16,
    pub timestamp: u32,
    pub marker: bool,
    pub payload_type: u8,
    pub extension: &'a [u8],
    pub payload: &'a [u8],
}
