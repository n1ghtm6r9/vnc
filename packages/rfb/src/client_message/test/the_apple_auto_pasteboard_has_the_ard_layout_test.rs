use crate::{ClientMessage, write_apple_auto_pasteboard};

#[test]
fn test_the_apple_auto_pasteboard_has_the_ard_layout() {
    let mut bytes = Vec::new();
    write_apple_auto_pasteboard(&mut bytes, true).unwrap();
    assert_eq!(bytes, vec![0x15, 0, 0, 1, 0, 0, 0, 0], "AutoPasteboard is the type, padding, the command and four zero bytes");
    assert_eq!(ClientMessage::read(&mut &bytes[..]).unwrap(), ClientMessage::AppleAutoPasteboard { enabled: true }, "the server side reads the command back");
}
