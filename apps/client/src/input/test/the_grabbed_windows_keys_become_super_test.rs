use winit::keyboard::{KeyCode, PhysicalKey};

use crate::physical_key_for_virtual_key;

#[test]
fn test_the_grabbed_windows_keys_become_super() {
    assert_eq!(physical_key_for_virtual_key(0x5b), Some(PhysicalKey::Code(KeyCode::SuperLeft)), "left Windows is left Super");
    assert_eq!(physical_key_for_virtual_key(0x5c), Some(PhysicalKey::Code(KeyCode::SuperRight)), "right Windows is right Super");
    assert_eq!(physical_key_for_virtual_key(0x41), None, "other keys never come through the grab");
}
