use rfb::{ClientMessage, Rect, write_framebuffer_update_header};

use crate::test_support::{
    FakeServer, read_client_messages, serve_until_connected, start_with_events, test_options,
    write_raw_rect,
};

#[test]
fn test_the_next_update_is_requested_before_the_current_one_is_decoded() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        let requests = read_client_messages(stream, 1);
        write_raw_rect(stream, Rect::new(0, 0, 64, 48), 0xFF00_00FF);
        requests
    });
    let (_session, _events) = start_with_events(test_options(server.port()));

    let requests = server.outcome();

    assert_eq!(
        requests,
        vec![ClientMessage::FramebufferUpdateRequest {
            incremental: true,
            rect: Rect::new(0, 0, 64, 48),
        }],
        "the client asks for the next update while the current one is still arriving"
    );
}
