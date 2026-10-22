use super::{GrabState, GrabbedKey};
use crate::should_grab;

impl GrabState {
    #[cfg_attr(not(windows), allow(dead_code))]
    pub(crate) fn decide(&mut self, key: GrabbedKey, foreground_is_ours: bool) -> bool {
        let held = self.swallowed.contains(&key.virtual_key);
        if !key.down {
            self.swallowed.retain(|virtual_key| *virtual_key != key.virtual_key);
            return held;
        }
        if held {
            return true;
        }
        let grab = should_grab(key.virtual_key, self.active, foreground_is_ours);
        if grab {
            self.swallowed.push(key.virtual_key);
        }
        grab
    }
}
