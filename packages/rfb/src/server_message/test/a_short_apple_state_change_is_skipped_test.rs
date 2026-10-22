use crate::ServerMessage;

#[test]
fn test_a_short_apple_state_change_is_skipped() {
    let bytes = [0x14u8, 0, 0, 2, 0, 4, 0x07];
    let mut reader = &bytes[..];
    assert_eq!(
        ServerMessage::read(&mut reader).unwrap(),
        ServerMessage::AppleStateChange { status: 0 },
        "a body too short for flags and status has no status"
    );
    assert_eq!(reader, &[0x07][..], "its bytes are still consumed");
}
