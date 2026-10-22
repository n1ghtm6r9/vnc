use crate::{AppleDisplayGeometry, ServerMessage};

#[test]
fn test_an_apple_display_info_reports_the_pixel_size() {
    let mut bytes = vec![0x51u8, 0, 20];
    bytes.extend_from_slice(&[0, 1, 0x05, 0xe8, 0x03, 0xd6, 0x0b, 0xd0, 0x07, 0xac]);
    bytes.extend_from_slice(&[0, 0, 0, 4, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        ServerMessage::read(&mut &bytes[..]).unwrap(),
        ServerMessage::AppleDisplayInfo {
            size: Some((3024, 1964)),
            geometry: Some(AppleDisplayGeometry {
                pixel_width: 3024,
                pixel_height: 1964,
                point_width: 1512,
                point_height: 982,
            }),
        },
        "the pixel size follows the size in points, as the Mac sent on 2026-10-21"
    );
}
