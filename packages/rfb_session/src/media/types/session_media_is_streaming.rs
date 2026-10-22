use std::sync::atomic::Ordering;

use super::SessionMedia;

impl SessionMedia {
    pub(crate) fn is_streaming(&self) -> bool {
        self.streaming.load(Ordering::Relaxed)
    }
}
