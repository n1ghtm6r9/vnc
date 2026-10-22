use std::sync::atomic::Ordering;

use super::SessionWriter;

impl SessionWriter {
    pub(super) fn media_streaming(&self) -> bool {
        self.streaming
            .as_ref()
            .is_some_and(|streaming| streaming.load(Ordering::Relaxed))
    }
}
