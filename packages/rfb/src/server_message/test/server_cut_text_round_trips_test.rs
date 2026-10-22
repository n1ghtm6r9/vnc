use crate::{ServerMessage, write_server_cut_text};

#[test]
fn test_server_cut_text_round_trips() {
    let mut bytes = Vec::new();
    write_server_cut_text(&mut bytes, b"copied on the Mac").unwrap();
    let message = ServerMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ServerMessage::ServerCutText { text: b"copied on the Mac".to_vec() }, "the clipboard bytes read back unchanged");
}
