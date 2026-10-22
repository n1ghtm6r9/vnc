use crate::{ClientMessage, write_client_cut_text};

#[test]
fn test_client_cut_text_round_trips() {
    let mut bytes = Vec::new();
    write_client_cut_text(&mut bytes, b"hello").unwrap();
    let message = ClientMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ClientMessage::ClientCutText { text: b"hello".to_vec() }, "the clipboard bytes read back unchanged");
}
