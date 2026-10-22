use crate::{AppleServerInit, PixelFormat, ServerInit};

#[test]
fn test_a_plain_server_init_read_the_apple_way_keeps_its_name() {
    let init = ServerInit {
        width: 800,
        height: 600,
        pixel_format: PixelFormat::rgba32(),
        name: "MacBook Pro".to_owned(),
    };
    let mut bytes = Vec::new();
    init.write(&mut bytes).unwrap();

    let apple = AppleServerInit::read(&mut &bytes[..]).unwrap();

    assert_eq!(apple.init, init, "a name without the Apple marker is the whole field");
    assert_eq!(apple.flags, 0, "a plain ServerInit has no server flags");
}
