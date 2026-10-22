use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{CommandKeysym, KeyMapping, keysym_for_physical_key};

#[test]
fn test_by_default_the_windows_key_is_super_and_control_stays_control() {
    let mapping = KeyMapping::default();
    let keysym = |code| keysym_for_physical_key(PhysicalKey::Code(code), mapping);
    assert!(!mapping.swap_control_and_command, "the default mapping does not swap control and command");
    assert_eq!(mapping.command_keysym, CommandKeysym::Super, "the default command keysym is Super");
    assert_eq!(keysym(KeyCode::SuperLeft), Some(0xffeb), "the left Windows key is Super_L, which the Mac reads as Command");
    assert_eq!(keysym(KeyCode::SuperRight), Some(0xffec), "the right Windows key is Super_R");
    assert_eq!(keysym(KeyCode::ControlLeft), Some(0xffe3), "the left Control key stays Control_L");
    assert_eq!(keysym(KeyCode::ControlRight), Some(0xffe4), "the right Control key stays Control_R");
}
