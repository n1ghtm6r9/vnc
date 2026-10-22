use crate::RateControl;

pub fn rate_control_packet(report: &RateControl) -> [u8; 32] {
    let mut packet = [0u8; 32];
    packet[0..4].copy_from_slice(&0x80cc_0007u32.to_be_bytes());
    packet[4..8].copy_from_slice(&report.ssrc.to_be_bytes());
    packet[8..12].copy_from_slice(b"RCTL");
    packet[12..16].copy_from_slice(&0x8500_0004u32.to_be_bytes());
    packet[16..18].copy_from_slice(&report.pictures.to_be_bytes());
    packet[22..24].copy_from_slice(&report.since_last_packet_ms.to_be_bytes());
    packet[24..26].copy_from_slice(&[0xff, 0xff]);
    packet[26..30].copy_from_slice(&report.packets.to_be_bytes());
    packet[30..32].copy_from_slice(&report.bandwidth_kbit.to_be_bytes());
    packet
}
