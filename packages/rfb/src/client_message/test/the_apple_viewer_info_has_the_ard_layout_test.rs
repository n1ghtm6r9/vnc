use crate::{ClientMessage, write_apple_viewer_info};

#[test]
fn test_the_apple_viewer_info_has_the_ard_layout() {
    let mut bytes = Vec::new();
    write_apple_viewer_info(&mut bytes).unwrap();
    let mut expected = vec![0x21, 0, 0, 62, 0, 1, 0, 0, 0, 2];
    expected.extend_from_slice(&[0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0, 0]);
    expected.extend_from_slice(&[0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut bitmap = [0u8; 32];
    bitmap[0] = 0xb0;
    bitmap[2] = 0x0c;
    bitmap[3] = 0x03;
    bitmap[4] = 0x90;
    bitmap[10] = 0x40;
    expected.extend_from_slice(&bitmap);
    assert_eq!(bytes, expected, "the 66 bytes noVNC-ARD sends and the Mac answered on 2026-10-21");
    assert_eq!(ClientMessage::read(&mut &bytes[..]).unwrap(), ClientMessage::AppleViewerInfo, "the server side reads it whole");
}
