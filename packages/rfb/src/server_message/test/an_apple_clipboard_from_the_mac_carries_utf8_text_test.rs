use crate::{ServerMessage, write_apple_clipboard};

#[test]
fn test_an_apple_clipboard_from_the_mac_carries_utf8_text() {
    let mut bytes = Vec::new();
    write_apple_clipboard(&mut bytes, 0x0102_0304, "Ёлка 🎄").unwrap();
    assert_eq!(
        ServerMessage::read(&mut &bytes[..]).unwrap(),
        ServerMessage::AppleClipboard { session_id: 0x0102_0304, text: Some("Ёлка 🎄".to_owned()) },
        "the Mac sends its pasteboard in the layout the client uses, with UTF-8 text"
    );
}
