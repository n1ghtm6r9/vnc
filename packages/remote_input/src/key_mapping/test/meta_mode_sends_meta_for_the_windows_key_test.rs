use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{CommandKeysym, KeyMapping, keysym_for_physical_key};

#[test]
fn test_meta_mode_sends_meta_for_the_windows_key() {
    let mapping = KeyMapping { swap_control_and_command: false, command_keysym: CommandKeysym::Meta };
    let keysym = |code| keysym_for_physical_key(PhysicalKey::Code(code), mapping);
    assert_eq!(keysym(KeyCode::SuperLeft), Some(0xffe7), "the left Windows key is Meta_L");
    assert_eq!(keysym(KeyCode::SuperRight), Some(0xffe8), "the right Windows key is Meta_R");
    assert_eq!(keysym(KeyCode::ControlLeft), Some(0xffe3), "Meta mode leaves the left Control key alone");
    assert_eq!(keysym(KeyCode::ControlRight), Some(0xffe4), "Meta mode leaves the right Control key alone");
}
