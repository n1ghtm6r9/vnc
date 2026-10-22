use crate::{AppleServerInit, PixelFormat, ServerInit};

#[test]
fn test_the_apple_server_init_round_trips() {
    let apple = AppleServerInit {
        init: ServerInit {
            width: 3024,
            height: 1964,
            pixel_format: PixelFormat::rgba32(),
            name: "MacBook Pro — Богдан".to_owned(),
        },
        flags: 0x52,
        capabilities: [0xbf, 0xf6, 0xe7, 0x2f, 0xec, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    };
    let mut bytes = Vec::new();
    apple.write(&mut bytes).unwrap();
    assert_eq!(
        AppleServerInit::read(&mut &bytes[..]).unwrap(),
        apple,
        "flags, capabilities and a non-ASCII name read back unchanged"
    );
}
