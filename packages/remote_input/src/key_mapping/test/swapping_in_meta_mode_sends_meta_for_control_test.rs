use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{CommandKeysym, KeyMapping, keysym_for_physical_key};

#[test]
fn test_swapping_in_meta_mode_sends_meta_for_control() {
    let mapping = KeyMapping { swap_control_and_command: true, command_keysym: CommandKeysym::Meta };
    let keysym = |code| keysym_for_physical_key(PhysicalKey::Code(code), mapping);
    assert_eq!(keysym(KeyCode::ControlLeft), Some(0xffe7), "the left Control key becomes Meta_L");
    assert_eq!(keysym(KeyCode::ControlRight), Some(0xffe8), "the right Control key becomes Meta_R");
    assert_eq!(keysym(KeyCode::SuperLeft), Some(0xffe3), "the left Windows key becomes Control_L");
    assert_eq!(keysym(KeyCode::SuperRight), Some(0xffe4), "the right Windows key becomes Control_R");
}
