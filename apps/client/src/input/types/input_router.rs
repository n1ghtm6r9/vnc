use remote_input::{KeyMapping, PressedKeys, WheelAccumulator};

pub struct InputRouter {
    pub(super) mapping: KeyMapping,
    pub(super) pressed: PressedKeys,
    pub(super) wheel: WheelAccumulator,
    pub(super) buttons: u8,
    pub(super) position: Option<(u16, u16)>,
    pub(super) apple_scroll: bool,
}
