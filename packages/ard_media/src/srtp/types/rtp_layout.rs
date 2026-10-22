#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtpLayout {
    pub header_length: usize,
    pub payload_offset: usize,
}
