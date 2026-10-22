use std::time::Instant;

use ard_media::{extend_seq, is_rtcp};

use super::{MediaStream, Received, SenderReport};
use crate::MediaChannel;

impl MediaStream {
    pub(crate) fn receive<'a>(
        &mut self,
        channel: MediaChannel,
        datagram: &'a mut [u8],
        now: Instant,
    ) -> Received<'a> {
        let index = channel.index();
        if is_rtcp(datagram) {
            let Some(plain) = self.incoming[index].unprotect_rtcp(datagram) else {
                return Received::Rejected;
            };
            if plain.len() >= 16 && plain[1] == 200 {
                self.sender_reports[index] = Some(SenderReport {
                    middle: u32::from_be_bytes([plain[10], plain[11], plain[12], plain[13]]),
                    at: now,
                });
            }
            return Received::Control;
        }
        let Some(packet) = self.incoming[index].unprotect_rtp(datagram) else {
            return Received::Rejected;
        };
        self.last_media_at = now;
        if channel == MediaChannel::Audio || packet.ssrc == self.answer.video.ssrc {
            self.highest_seq[index] = Some(
                self.highest_seq[index].map_or(i64::from(packet.seq), |highest| {
                    highest.max(extend_seq(highest, packet.seq))
                }),
            );
        }
        if channel == MediaChannel::Audio {
            return Received::Audio(packet);
        }
        self.last_video_packet_at = Some(now);
        Received::Video(packet)
    }
}
