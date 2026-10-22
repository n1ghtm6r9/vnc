use winit::keyboard::{NativeKeyCode, PhysicalKey};

use crate::{CommandKeysym, KeyMapping, keysym_for_physical_key};

#[test]
fn test_an_unidentified_physical_key_sends_nothing() {
    let key = PhysicalKey::Unidentified(NativeKeyCode::Unidentified);
    let swapped_meta = KeyMapping { swap_control_and_command: true, command_keysym: CommandKeysym::Meta };
    assert_eq!(keysym_for_physical_key(key, KeyMapping::default()), None, "a key winit cannot name has no keysym");
    assert_eq!(keysym_for_physical_key(key, swapped_meta), None, "no mapping invents a keysym for an unknown key");
}
