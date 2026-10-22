use winit::keyboard::{KeyCode, PhysicalKey};

use crate::PressedKeys;

#[test]
fn test_a_repeated_press_keeps_the_keysym_of_the_first_press() {
    let mut keys = PressedKeys::new();
    let q = PhysicalKey::Code(KeyCode::KeyQ);
    assert_eq!(
        keys.press(q, 0x0100_0439),
        0x0100_0439,
        "the first press sends its own keysym"
    );
    assert_eq!(
        keys.press(q, 0x0100_0419),
        0x0100_0439,
        "an auto-repeat after Shift went down repeats the keysym the Mac holds"
    );
}
