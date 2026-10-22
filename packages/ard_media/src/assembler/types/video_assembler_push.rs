use std::time::Instant;

use super::{AssemblerEvent, VideoAssembler};
use crate::RtpPacket;

impl VideoAssembler {
    pub fn push(&mut self, packet: &RtpPacket<'_>, now: Instant, events: &mut Vec<AssemblerEvent>) {
        self.stats.packets += 1;
        if self.stalled {
            self.stats.ignored += 1;
            return;
        }
        self.collect(packet, now, events);
        self.check_stall(now, events);
    }
}
