use crate::{ClientMessage, write_pointer_event};

#[test]
fn test_a_pointer_event_has_the_spec_layout() {
    let mut bytes = Vec::new();
    write_pointer_event(&mut bytes, 0b101, 3455, 2233).unwrap();
    assert_eq!(bytes, vec![5, 0b101, 0x0d, 0x7f, 0x08, 0xb9], "type, button mask, then x and y big-endian");
    let message = ClientMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ClientMessage::PointerEvent { buttons: 0b101, x: 3455, y: 2233 }, "the server side reads the same pointer back");
}
