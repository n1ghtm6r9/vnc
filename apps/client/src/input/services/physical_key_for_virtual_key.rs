use platform::{LEFT_WINDOWS_KEY, RIGHT_WINDOWS_KEY};
use winit::keyboard::{KeyCode, PhysicalKey};

pub fn physical_key_for_virtual_key(virtual_key: u32) -> Option<PhysicalKey> {
    match virtual_key {
        LEFT_WINDOWS_KEY => Some(PhysicalKey::Code(KeyCode::SuperLeft)),
        RIGHT_WINDOWS_KEY => Some(PhysicalKey::Code(KeyCode::SuperRight)),
        _ => None,
    }
}
