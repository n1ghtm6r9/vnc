use std::sync::atomic::Ordering;

use super::{MediaHandle, MediaInput};

impl Drop for MediaHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.inputs.send(MediaInput::Stop);
    }
}
