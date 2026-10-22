use crate::{ServerMessage, write_bell};

#[test]
fn test_a_bell_is_one_byte() {
    let mut bytes = Vec::new();
    write_bell(&mut bytes).unwrap();
    assert_eq!(bytes, vec![2], "a bell has no body");
    assert_eq!(ServerMessage::read(&mut &bytes[..]).unwrap(), ServerMessage::Bell, "the client reads a bell back");
}
