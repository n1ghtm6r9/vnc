use std::time::Instant;

use super::PendingOffer;
use crate::MediaStream;

pub(crate) enum MediaPhase {
    AwaitingMessage1 {
        deadline: Instant,
    },
    Offering(PendingOffer),
    Waiting {
        stream: Box<MediaStream>,
        deadline: Instant,
    },
    Streaming {
        stream: Box<MediaStream>,
        reoffer: Option<PendingOffer>,
    },
    Finished,
}
