use ard_media::extend_seq;

use super::{AudioReorder, OrderedFrame};
use crate::{STALE_LIMIT, SoundPacket};

impl AudioReorder {
    pub(crate) fn push(&mut self, packet: SoundPacket) -> Vec<OrderedFrame> {
        if self.ssrc != Some(packet.ssrc) || self.stale >= STALE_LIMIT {
            *self = AudioReorder {
                ssrc: Some(packet.ssrc),
                ..AudioReorder::default()
            };
        }
        let seq = self.highest.map_or(i64::from(packet.seq), |highest| {
            extend_seq(highest, packet.seq)
        });
        let next = *self.next.get_or_insert(seq);
        if seq < next || self.waiting.contains_key(&seq) {
            self.stale += 1;
            return Vec::new();
        }
        self.stale = 0;
        self.highest = Some(self.highest.map_or(seq, |highest| highest.max(seq)));
        self.waiting.insert(seq, packet.payload);
        self.drain(next)
    }
}
