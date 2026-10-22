use std::time::Instant;

use crate::Snapshot;

impl Snapshot {
    pub(crate) fn new(started: Instant) -> Snapshot {
        Snapshot {
            started,
            width: 0,
            height: 0,
            pixels: Vec::new(),
            first_frame: None,
            stats: None,
        }
    }
}
