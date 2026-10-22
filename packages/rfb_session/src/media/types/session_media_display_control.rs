use std::sync::Arc;

use super::{DisplayControl, SessionMedia};

impl SessionMedia {
    pub(crate) fn display_control(&self, scale: u32) -> DisplayControl {
        DisplayControl {
            peer: Arc::clone(&self.peer),
            display: self.display.clone(),
            scale,
        }
    }
}
