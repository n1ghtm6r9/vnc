use winit::keyboard::{KeyCode, PhysicalKey};

use crate::PressedKeys;

#[test]
fn test_drained_keys_come_back_in_the_order_they_were_pressed() {
    let mut keys = PressedKeys::new();
    for (code, keysym) in [
        (KeyCode::SuperLeft, 0xffeb),
        (KeyCode::KeyC, 0x63),
        (KeyCode::ShiftLeft, 0xffe1),
    ] {
        keys.press(PhysicalKey::Code(code), keysym);
    }
    assert_eq!(
        keys.drain(),
        vec![0xffeb, 0x63, 0xffe1],
        "key-ups go out in the order the keys went down, which keeps the output deterministic"
    );
}
