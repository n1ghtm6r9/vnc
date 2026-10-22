use std::time::Instant;

use ard_media::{IDLE_BANDWIDTH_KBIT, RateControl, rate_control_packet, receiver_report_packet};

use super::MediaSession;
use crate::{FrameDecoder, MEDIA_CHANNELS, MediaChannel, RATE_CONTROL_INTERVAL, REPORT_INTERVAL};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn send_feedback(&mut self, now: Instant) {
        let Some(stream) = self.phase.stream_mut() else {
            return;
        };
        if now >= self.next_rate_control {
            self.next_rate_control = now + RATE_CONTROL_INTERVAL;
            let since_last_packet = stream
                .last_video_packet_at
                .map_or(0, |at| now.saturating_duration_since(at).as_millis());
            let report = RateControl {
                ssrc: stream.secrets.video_ssrc,
                pictures: self.pictures as u16,
                since_last_packet_ms: u16::try_from(since_last_packet).unwrap_or(u16::MAX),
                packets: self.video_packets,
                bandwidth_kbit: IDLE_BANDWIDTH_KBIT,
            };
            self.effects
                .datagrams
                .extend(stream.protect(MediaChannel::Video, &rate_control_packet(&report)));
        }
        if now >= self.next_report {
            self.next_report = now + REPORT_INTERVAL;
            for channel in MEDIA_CHANNELS {
                if let Some(report) = stream.receiver_report(channel, now) {
                    self.effects
                        .datagrams
                        .extend(stream.protect(channel, &receiver_report_packet(&report)));
                }
            }
        }
    }
}
