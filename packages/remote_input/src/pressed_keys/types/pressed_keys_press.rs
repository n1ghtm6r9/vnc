use winit::keyboard::PhysicalKey;

use super::PressedKeys;

impl PressedKeys {
    pub fn press(&mut self, key: PhysicalKey, keysym: u32) -> u32 {
        if let Some(&(_, held)) = self.keys.iter().find(|(pressed, _)| *pressed == key) {
            return held;
        }
        self.keys.push((key, keysym));
        keysym
    }
}
