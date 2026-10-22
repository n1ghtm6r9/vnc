use crate::{AppleServerInit, PixelFormat};

#[test]
fn test_the_extended_apple_server_init_splits_flags_capabilities_and_name() {
    let mut bytes = vec![0x0b, 0xd0, 0x07, 0xac];
    PixelFormat::rgba32().write(&mut bytes).unwrap();
    let mut field = vec![0, 0, 0, 0, 0, 0x52];
    field.extend_from_slice(&[0xbf, 0xf6, 0xe7, 0x2f, 0xec, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    field.push(0);
    field.extend_from_slice("Bogdan’s MacBook Pro".as_bytes());
    bytes.extend_from_slice(&(field.len() as u32).to_be_bytes());
    bytes.extend_from_slice(&field);

    let apple = AppleServerInit::read(&mut &bytes[..]).unwrap();

    assert_eq!((apple.init.width, apple.init.height), (3024, 1964), "the size comes first, as in plain RFB");
    assert_eq!(apple.flags, 0x52, "bytes 2 to 5 of the name field are the server flags");
    assert_eq!(
        apple.capabilities[..5],
        [0xbf, 0xf6, 0xe7, 0x2f, 0xec],
        "bytes 6 to 21 of the name field are the capability bitmap"
    );
    assert_eq!(apple.init.name, "Bogdan’s MacBook Pro", "the name is what follows the last NUL");
}
