use super::KeyboardGrab;
use crate::lock_active_grab;

impl KeyboardGrab {
    pub fn set_active(&self, active: bool) {
        if let Some(state) = lock_active_grab().as_mut() {
            state.active = active;
        }
    }
}
