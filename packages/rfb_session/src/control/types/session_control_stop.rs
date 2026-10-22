use std::sync::atomic::Ordering;

use super::SessionControl;

impl SessionControl {
    pub(crate) fn stop(&self) {
        self.stopped.store(true, Ordering::SeqCst);
    }
}
