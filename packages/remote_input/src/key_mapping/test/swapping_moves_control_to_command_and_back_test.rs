use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{CommandKeysym, KeyMapping, keysym_for_physical_key};

#[test]
fn test_swapping_moves_control_to_command_and_back() {
    let mapping = KeyMapping { swap_control_and_command: true, command_keysym: CommandKeysym::Super };
    let keysym = |code| keysym_for_physical_key(PhysicalKey::Code(code), mapping);
    assert_eq!(keysym(KeyCode::ControlLeft), Some(0xffeb), "the left Control key becomes Command");
    assert_eq!(keysym(KeyCode::ControlRight), Some(0xffec), "the right Control key becomes Command");
    assert_eq!(keysym(KeyCode::SuperLeft), Some(0xffe3), "the left Windows key becomes Control");
    assert_eq!(keysym(KeyCode::SuperRight), Some(0xffe4), "the right Windows key becomes Control");
}
