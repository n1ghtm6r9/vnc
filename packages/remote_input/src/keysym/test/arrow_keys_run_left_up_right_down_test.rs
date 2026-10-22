use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_arrow_keys_run_left_up_right_down() {
    let rows = [
        (KeyCode::ArrowLeft, 0xff51),
        (KeyCode::ArrowUp, 0xff52),
        (KeyCode::ArrowRight, 0xff53),
        (KeyCode::ArrowDown, 0xff54),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends its arrow keysym in the order left, up, right, down");
    }
}
