use crate::{APPLE_MODE_CONTROL, ClientMessage, write_apple_set_mode};

#[test]
fn test_the_apple_control_mode_has_the_ard_layout() {
    let mut bytes = Vec::new();
    write_apple_set_mode(&mut bytes, APPLE_MODE_CONTROL).unwrap();
    assert_eq!(bytes, vec![0x0a, 0, 0, 1], "SetMode is the type, two padding bytes and the mode, 1 for control");
    assert_eq!(ClientMessage::read(&mut &bytes[..]).unwrap(), ClientMessage::AppleSetMode { mode: 1 }, "the server side reads the mode back");
}
