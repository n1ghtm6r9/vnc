use std::{mem, time::Instant};

use ard_media::{parse_answer, parse_stream_ports};
use log::{debug, info};

use super::{MediaMeter, MediaPhase, MediaSession};
use crate::{
    FIRST_PICTURE_TIMEOUT, FrameDecoder, MediaFailure, MediaStream, RATE_CONTROL_INTERVAL,
    REPORT_INTERVAL,
};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn handle_answer(&mut self, payload: &[u8], now: Instant) {
        if let Some(ports) = parse_stream_ports(payload) {
            debug!(
                "[MEDIA] the Mac streams from ports {} and {}",
                ports.audio_port, ports.video_port
            );
            return;
        }
        let (pending, first) = match mem::replace(&mut self.phase, MediaPhase::Finished) {
            MediaPhase::Offering(offer) => (offer, true),
            MediaPhase::Streaming {
                reoffer: Some(offer),
                ..
            } => (offer, false),
            other => {
                self.phase = other;
                debug!("[MEDIA] a media answer nobody waits for is ignored");
                return;
            }
        };
        let answer = match parse_answer(payload) {
            Ok(answer) => answer,
            Err(error) => {
                self.fail(MediaFailure::BadAnswer(error.to_string()));
                return;
            }
        };
        info!(
            "[MEDIA] the Mac answers with {}x{} video in {} stripes",
            answer.video.width, answer.video.height, answer.video.stripes
        );
        let stream = Box::new(MediaStream::new(pending.secrets, answer, now));
        self.phase = if first {
            self.next_rate_control = now + RATE_CONTROL_INTERVAL;
            self.next_report = now + REPORT_INTERVAL;
            self.meter = MediaMeter::new(now);
            MediaPhase::Waiting {
                stream,
                deadline: now + FIRST_PICTURE_TIMEOUT,
            }
        } else {
            MediaPhase::Streaming {
                stream,
                reoffer: None,
            }
        };
        for mut datagram in pending.early {
            self.handle_datagram(datagram.channel, &mut datagram.bytes, now);
        }
    }
}
