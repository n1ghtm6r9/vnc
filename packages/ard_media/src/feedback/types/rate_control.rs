#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RateControl {
    pub ssrc: u32,
    pub pictures: u16,
    pub since_last_packet_ms: u16,
    pub packets: u32,
    pub bandwidth_kbit: u16,
}
