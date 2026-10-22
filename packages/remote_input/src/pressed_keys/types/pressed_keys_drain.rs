use std::mem;

use super::PressedKeys;

impl PressedKeys {
    pub fn drain(&mut self) -> Vec<u32> {
        mem::take(&mut self.keys)
            .into_iter()
            .map(|(_, keysym)| keysym)
            .collect()
    }
}
