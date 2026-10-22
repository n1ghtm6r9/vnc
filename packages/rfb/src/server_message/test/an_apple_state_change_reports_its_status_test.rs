use crate::ServerMessage;

#[test]
fn test_an_apple_state_change_reports_its_status() {
    let bytes = [0x14u8, 0, 0, 6, 0, 1, 0, 4, 9, 9, 0x07];
    let mut reader = &bytes[..];
    assert_eq!(
        ServerMessage::read(&mut reader).unwrap(),
        ServerMessage::AppleStateChange { status: 4 },
        "the status follows the flags, and the bytes past them belong to the message"
    );
    assert_eq!(reader, &[0x07][..], "the next message starts right after the declared size");
}
