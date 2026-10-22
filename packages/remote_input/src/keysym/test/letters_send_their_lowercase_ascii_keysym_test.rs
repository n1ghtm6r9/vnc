use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_letters_send_their_lowercase_ascii_keysym() {
    let rows = [
        (KeyCode::KeyA, 0x61),
        (KeyCode::KeyB, 0x62),
        (KeyCode::KeyC, 0x63),
        (KeyCode::KeyD, 0x64),
        (KeyCode::KeyE, 0x65),
        (KeyCode::KeyF, 0x66),
        (KeyCode::KeyG, 0x67),
        (KeyCode::KeyH, 0x68),
        (KeyCode::KeyI, 0x69),
        (KeyCode::KeyJ, 0x6a),
        (KeyCode::KeyK, 0x6b),
        (KeyCode::KeyL, 0x6c),
        (KeyCode::KeyM, 0x6d),
        (KeyCode::KeyN, 0x6e),
        (KeyCode::KeyO, 0x6f),
        (KeyCode::KeyP, 0x70),
        (KeyCode::KeyQ, 0x71),
        (KeyCode::KeyR, 0x72),
        (KeyCode::KeyS, 0x73),
        (KeyCode::KeyT, 0x74),
        (KeyCode::KeyU, 0x75),
        (KeyCode::KeyV, 0x76),
        (KeyCode::KeyW, 0x77),
        (KeyCode::KeyX, 0x78),
        (KeyCode::KeyY, 0x79),
        (KeyCode::KeyZ, 0x7a),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends its lowercase ASCII letter, which shortcuts use in any layout");
    }
}
