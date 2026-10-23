use std::{mem, time::Instant};

use ard_media::{is_answer, is_ports_notice, parse_answer};
use log::{debug, info, warn};

use super::{MediaMeter, MediaPhase, MediaSession};
use crate::{
    FIRST_PICTURE_TIMEOUT, FrameDecoder, MediaFailure, MediaStream, RATE_CONTROL_INTERVAL,
    REPORT_INTERVAL, log_ports_notice, logged_bytes,
};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn handle_answer(&mut self, payload: &[u8], now: Instant) {
        if is_ports_notice(payload) {
            log_ports_notice(payload);
            return;
        }
        if !is_answer(payload) {
            warn!(
                "[MEDIA] the Mac sent a media notice of another kind: {:02x?}",
                logged_bytes(payload)
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
                warn!(
                    "[MEDIA] the Mac's unreadable media answer: {:02x?}",
                    logged_bytes(payload)
                );
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
