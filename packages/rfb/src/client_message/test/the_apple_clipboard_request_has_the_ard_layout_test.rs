use crate::{ClientMessage, write_apple_clipboard_request};

#[test]
fn test_the_apple_clipboard_request_has_the_ard_layout() {
    let mut bytes = Vec::new();
    write_apple_clipboard_request(&mut bytes, 0x0a0b_0c0d).unwrap();
    assert_eq!(bytes, vec![0x0b, 0, 0, 0, 0x0a, 0x0b, 0x0c, 0x0d], "ClipboardRequest is the type, format 0, padding and the session id");
    assert_eq!(
        ClientMessage::read(&mut &bytes[..]).unwrap(),
        ClientMessage::AppleClipboardRequest { session_id: 0x0a0b_0c0d },
        "the server side reads the session id back"
    );
}
