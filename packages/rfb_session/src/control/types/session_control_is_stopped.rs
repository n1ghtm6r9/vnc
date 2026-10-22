use std::sync::atomic::Ordering;

use super::SessionControl;

impl SessionControl {
    pub(crate) fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }
}
