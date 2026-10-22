use super::PressedKeys;

impl PressedKeys {
    pub const fn new() -> PressedKeys {
        PressedKeys { keys: Vec::new() }
    }
}
