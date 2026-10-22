use crate::ServerMessage;

#[test]
fn test_apple_acks_and_nops_carry_nothing() {
    let bytes = [0x04u8, 0x07];
    let mut reader = &bytes[..];
    assert_eq!(ServerMessage::read(&mut reader).unwrap(), ServerMessage::AppleAck, "0x04 is a bare acknowledgement");
    assert_eq!(ServerMessage::read(&mut reader).unwrap(), ServerMessage::AppleNop, "0x07 is a bare keep-alive");
    assert!(reader.is_empty(), "neither message has a body");
}
