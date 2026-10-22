use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{
    KEYSYM_META_LEFT, KEYSYM_META_RIGHT, KEYSYM_SUPER_LEFT, KEYSYM_SUPER_RIGHT, PressedKeys,
};

#[test]
fn test_a_held_command_control_or_option_makes_a_shortcut() {
    let mut shifted = PressedKeys::new();
    shifted.press(PhysicalKey::Code(KeyCode::ShiftLeft), 0xffe1);
    assert!(
        !shifted.holds_shortcut_modifier(),
        "Shift alone only changes the character"
    );
    for keysym in [
        0xffe3,
        0xffe4,
        0xffe9,
        0xffea,
        KEYSYM_SUPER_LEFT,
        KEYSYM_SUPER_RIGHT,
        KEYSYM_META_LEFT,
        KEYSYM_META_RIGHT,
    ] {
        let mut keys = PressedKeys::new();
        keys.press(PhysicalKey::Code(KeyCode::ControlLeft), keysym);
        assert!(
            keys.holds_shortcut_modifier(),
            "{keysym:#x} held on the Mac turns the next key into a shortcut"
        );
    }
}
