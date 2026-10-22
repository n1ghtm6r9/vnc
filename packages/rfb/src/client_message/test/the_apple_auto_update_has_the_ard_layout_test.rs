use crate::{ClientMessage, Rect, write_apple_auto_update};

#[test]
fn test_the_apple_auto_update_has_the_ard_layout() {
    let mut bytes = Vec::new();
    write_apple_auto_update(&mut bytes, Rect::new(0, 0, 3024, 1964)).unwrap();
    assert_eq!(
        bytes,
        vec![0x09, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0x0b, 0xd0, 0x07, 0xac],
        "AutoFBUpdate turns pushing on with no interval for the whole screen"
    );
    assert_eq!(
        ClientMessage::read(&mut &bytes[..]).unwrap(),
        ClientMessage::AppleAutoUpdate { enabled: true, rect: Rect::new(0, 0, 3024, 1964) },
        "the server side reads the area back"
    );
}
