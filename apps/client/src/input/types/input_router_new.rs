use remote_input::{KeyMapping, PressedKeys, WheelAccumulator};

use super::InputRouter;

impl InputRouter {
    pub fn new(mapping: KeyMapping) -> InputRouter {
        InputRouter {
            mapping,
            pressed: PressedKeys::new(),
            wheel: WheelAccumulator::new(),
            buttons: 0,
            position: None,
            apple_scroll: false,
        }
    }
}
