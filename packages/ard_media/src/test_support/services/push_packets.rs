use std::time::Instant;

use crate::{AssemblerEvent, VideoAssembler, test_support::TestPacket};

pub fn push_packets(
    assembler: &mut VideoAssembler,
    packets: &[TestPacket],
    now: Instant,
) -> Vec<AssemblerEvent> {
    let mut events = Vec::new();
    for packet in packets {
        assembler.push(&packet.rtp(), now, &mut events);
    }
    events
}
