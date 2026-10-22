use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{CommandKeysym, KeyMapping, keysym_for_key_code, keysym_for_physical_key};

#[test]
fn test_other_keys_ignore_the_mapping() {
    let mapping = KeyMapping { swap_control_and_command: true, command_keysym: CommandKeysym::Meta };
    let untouched = [
        KeyCode::KeyA,
        KeyCode::Enter,
        KeyCode::ShiftLeft,
        KeyCode::AltLeft,
        KeyCode::AltRight,
        KeyCode::CapsLock,
        KeyCode::Fn,
    ];
    for code in untouched {
        assert_eq!(keysym_for_physical_key(PhysicalKey::Code(code), mapping), keysym_for_key_code(code), "{code:?} is not Control or the Windows key, so the mapping leaves it alone");
    }
}
