use crate::{PixelFormat, ServerInit};

#[test]
fn test_the_server_init_round_trips() {
    let init = ServerInit {
        width: 3456,
        height: 2234,
        pixel_format: PixelFormat::rgba32(),
        name: "MacBook Pro — Богдан".to_string(),
    };
    let mut bytes = Vec::new();
    init.write(&mut bytes).unwrap();
    assert_eq!(ServerInit::read(&mut &bytes[..]).unwrap(), init, "size, format and a non-ASCII name read back unchanged");
}
