use std::{sync::mpsc::Receiver, time::Instant};

use crate::{MediaEvent, test_support::EVENT_TIMEOUT};

pub fn next_event(
    events: &Receiver<MediaEvent>,
    wanted: impl Fn(&MediaEvent) -> bool,
) -> MediaEvent {
    let deadline = Instant::now() + EVENT_TIMEOUT;
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        let event = events
            .recv_timeout(left)
            .expect("the media sends the awaited event in time");
        if wanted(&event) {
            return event;
        }
    }
}
