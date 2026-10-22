use std::time::Instant;

use super::{AssemblerEvent, PendingPicture, VideoAssembler};
use crate::{RtpPacket, assemble_picture, parse_payload};

impl VideoAssembler {
    pub(crate) fn collect(
        &mut self,
        packet: &RtpPacket<'_>,
        now: Instant,
        events: &mut Vec<AssemblerEvent>,
    ) {
        let stripe = packet.ssrc.wrapping_sub(self.base_ssrc);
        let [_, _, _, _, count_high, count_low, frame_high, frame_low] = packet.extension else {
            self.stats.ignored += 1;
            return;
        };
        if stripe >= self.stripes {
            self.stats.ignored += 1;
            return;
        }
        let count = u16::from_be_bytes([*count_high, *count_low]);
        let frame = u16::from_be_bytes([*frame_high, *frame_low]);
        let Some(parsed) = parse_payload(packet.payload).filter(|_| count > 0) else {
            self.stats.malformed += 1;
            return;
        };
        let don = self.unwrap_don(parsed.don);
        if let Some(next) = self.next
            && don < next
        {
            if self.first.is_some_and(|first| don >= first) {
                self.stats.duplicates += 1;
            } else {
                self.stats.ignored += 1;
            }
            return;
        }
        let picture = self
            .pictures
            .entry(don)
            .or_insert_with(|| PendingPicture::new(stripe, frame, count, now));
        if picture.stripe != stripe || picture.count != count {
            self.stats.malformed += 1;
            return;
        }
        if picture.data.is_some() || picture.parts.contains_key(&packet.seq) {
            self.stats.duplicates += 1;
            return;
        }
        if picture.parts.len() >= usize::from(picture.count) {
            self.stats.malformed += 1;
            return;
        }
        picture.parts.insert(packet.seq, parsed.part);
        if picture.parts.len() < usize::from(picture.count) {
            return;
        }
        let Some((data, key)) = assemble_picture(&picture.parts) else {
            self.stats.malformed += 1;
            return;
        };
        picture.parts = Default::default();
        picture.data = Some(data);
        picture.key = key;
        picture.done_at = Some(now);
        if self.next.is_none() && key {
            self.start(don);
        }
        self.emit(events);
    }
}
