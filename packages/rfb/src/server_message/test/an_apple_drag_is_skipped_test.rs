use crate::ServerMessage;

#[test]
fn test_an_apple_drag_is_skipped() {
    let bytes = [0x20u8, 0, 0, 0, 1, 0, 0, 0, 3, 1, 2, 3, 0x07];
    let mut reader = &bytes[..];
    assert_eq!(ServerMessage::read(&mut reader).unwrap(), ServerMessage::AppleDrag, "a drag from the Mac is read whole");
    assert_eq!(reader, &[0x07][..], "its extra bytes are consumed");
}
