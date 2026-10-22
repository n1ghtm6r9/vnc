use crate::{ClientMessage, Rect, write_framebuffer_update_request};

#[test]
fn test_an_update_request_has_the_spec_layout() {
    let mut bytes = Vec::new();
    let rect = Rect::new(0, 0, 3456, 2234);
    write_framebuffer_update_request(&mut bytes, false, rect).unwrap();
    assert_eq!(bytes, vec![3, 0, 0, 0, 0, 0, 0x0d, 0x80, 0x08, 0xba], "type, incremental flag, then x, y, width, height");
    let message = ClientMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ClientMessage::FramebufferUpdateRequest { incremental: false, rect }, "the server side reads the same request back");
}
