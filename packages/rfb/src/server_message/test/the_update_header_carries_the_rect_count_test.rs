use crate::{ServerMessage, write_framebuffer_update_header};

#[test]
fn test_the_update_header_carries_the_rect_count() {
    let mut bytes = Vec::new();
    write_framebuffer_update_header(&mut bytes, 0xffff).unwrap();
    assert_eq!(bytes, vec![0, 0, 0xff, 0xff], "type, padding, then the count big-endian");
    let message = ServerMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(message, ServerMessage::FramebufferUpdate { rect_count: 0xffff }, "0xffff is the count a LastRect update uses");
}
