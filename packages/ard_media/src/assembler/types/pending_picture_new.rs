use std::{collections::HashMap, time::Instant};

use super::PendingPicture;

impl PendingPicture {
    pub(crate) fn new(stripe: u32, frame: u16, count: u16, now: Instant) -> Self {
        Self {
            stripe,
            frame,
            count,
            parts: HashMap::with_capacity(usize::from(count)),
            data: None,
            key: false,
            first_at: now,
            done_at: None,
        }
    }
}
