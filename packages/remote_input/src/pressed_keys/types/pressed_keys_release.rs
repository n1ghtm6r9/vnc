use winit::keyboard::PhysicalKey;

use super::PressedKeys;

impl PressedKeys {
    pub fn release(&mut self, key: PhysicalKey) -> Option<u32> {
        let index = self.keys.iter().position(|(pressed, _)| *pressed == key)?;
        Some(self.keys.remove(index).1)
    }
}
