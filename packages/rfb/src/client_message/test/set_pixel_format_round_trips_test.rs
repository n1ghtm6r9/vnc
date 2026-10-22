use crate::{ClientMessage, PixelFormat, write_set_pixel_format};

#[test]
fn test_set_pixel_format_round_trips() {
    let mut bytes = Vec::new();
    write_set_pixel_format(&mut bytes, &PixelFormat::rgba32()).unwrap();
    assert_eq!(bytes.len(), 20, "type, three padding bytes, sixteen bytes of format");
    let message = ClientMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ClientMessage::SetPixelFormat(PixelFormat::rgba32()), "the server side reads the format back");
}
