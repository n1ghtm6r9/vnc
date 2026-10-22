use crate::{ClientMessage, write_apple_encrypted_event};

#[test]
fn test_an_apple_encrypted_event_carries_the_block() {
    let block: [u8; 16] = std::array::from_fn(|index| index as u8 + 1);
    let mut bytes = Vec::new();
    write_apple_encrypted_event(&mut bytes, &block).unwrap();
    let mut expected = vec![0x10, 0];
    expected.extend_from_slice(&block);
    assert_eq!(bytes, expected, "EncryptedEvent is the type, zero flags and the 16 encrypted bytes");
    assert_eq!(ClientMessage::read(&mut &bytes[..]).unwrap(), ClientMessage::AppleEncryptedEvent { block }, "the server side reads the block back");
}
