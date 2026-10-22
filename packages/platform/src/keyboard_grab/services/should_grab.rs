use crate::{LEFT_WINDOWS_KEY, RIGHT_WINDOWS_KEY};

pub fn should_grab(virtual_key: u32, active: bool, foreground_is_ours: bool) -> bool {
    active && foreground_is_ours && (virtual_key == LEFT_WINDOWS_KEY || virtual_key == RIGHT_WINDOWS_KEY)
}
