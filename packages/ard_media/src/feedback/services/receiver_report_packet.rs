use crate::ReceiverReport;

pub fn receiver_report_packet(report: &ReceiverReport) -> [u8; 44] {
    let mut packet = [0u8; 44];
    let words = [
        0x81c9_0007,
        report.ssrc,
        report.source_ssrc,
        (u32::from(report.fraction_lost) << 24) | report.cumulative_lost.min(0x00ff_ffff),
        report.highest_seq,
        report.jitter,
        report.last_sender_report,
        report.delay_since_sender_report,
        0x81ca_0002,
        report.ssrc,
        0x0100_0000,
    ];
    let (slots, _) = packet.as_chunks_mut::<4>();
    for (slot, word) in slots.iter_mut().zip(words) {
        *slot = word.to_be_bytes();
    }
    packet
}
