use crate::ServerMessage;

#[test]
fn test_a_short_apple_display_info_has_no_size() {
    let bytes = [0x51u8, 0, 4, 0, 1, 0x05, 0xe8, 0x07];
    let mut reader = &bytes[..];
    assert_eq!(
        ServerMessage::read(&mut reader).unwrap(),
        ServerMessage::AppleDisplayInfo { size: None, geometry: None },
        "a payload shorter than the 20-byte header says nothing about the size"
    );
    assert_eq!(reader, &[0x07][..], "its bytes are still consumed");
}
