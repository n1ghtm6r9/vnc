use crate::{ClientMessage, write_key_event};

#[test]
fn test_a_key_event_has_the_spec_layout() {
    let mut bytes = Vec::new();
    write_key_event(&mut bytes, true, 0xff0d).unwrap();
    assert_eq!(bytes, vec![4, 1, 0, 0, 0, 0, 0xff, 0x0d], "type, down flag, two padding bytes, then the keysym big-endian");
    let message = ClientMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ClientMessage::KeyEvent { down: true, keysym: 0xff0d }, "the server side reads the same key back");
}
