use std::time::Instant;

use log::{info, warn};

use super::{MediaPhase, MediaSession};
use crate::{FrameDecoder, MediaFailure, RECOVERY_LIMIT, RECOVERY_WINDOW};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn glitch(&mut self, failure: MediaFailure, now: Instant) {
        let diagnosis = match &self.phase {
            MediaPhase::Streaming {
                stream,
                reoffer: None,
            } => stream.diagnosis(now),
            MediaPhase::Streaming {
                reoffer: Some(_), ..
            }
            | MediaPhase::Finished => return,
            MediaPhase::AwaitingMessage1 { .. }
            | MediaPhase::Offering(_)
            | MediaPhase::Waiting { .. } => {
                self.fail(failure);
                return;
            }
        };
        warn!("[MEDIA] {failure}: {diagnosis}");
        self.recoveries
            .retain(|at| now.saturating_duration_since(*at) < RECOVERY_WINDOW);
        if self.recoveries.len() >= RECOVERY_LIMIT {
            self.fail(failure);
            return;
        }
        self.recoveries.push(now);
        info!(
            "[MEDIA] asking the Mac for a fresh stream, attempt {} of {RECOVERY_LIMIT}",
            self.recoveries.len()
        );
        if let Some(offer) = self.offer(now)
            && let MediaPhase::Streaming { reoffer, .. } = &mut self.phase
        {
            *reoffer = Some(offer);
        }
    }
}
