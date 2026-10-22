use std::sync::mpsc;

use rfb::{
    ClientMessage, ENCODING_DESKTOP_SIZE, Rect, RectHeader, write_framebuffer_update_header,
};

use crate::ClientCommand;
use crate::test_support::{
    EVENT_TIMEOUT, FakeServer, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};

#[test]
fn test_refresh_asks_for_the_whole_desktop_after_a_resize() {
    let (refetched, refetch) = mpsc::channel();
    let server = FakeServer::start(move |stream| {
        serve_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 80, 60),
            encoding: ENCODING_DESKTOP_SIZE,
        }
        .write(stream)
        .expect("the fake server announces a new desktop size");
        read_client_messages(stream, 2);
        let _ = refetched.send(());
        read_client_messages(stream, 1)
    });
    let (session, _events) = start_with_events(test_options(server.port()));
    refetch
        .recv_timeout(EVENT_TIMEOUT)
        .expect("the client asks for the resized desktop");

    session.send(ClientCommand::RefreshScreen);
    let requests = server.outcome();

    assert_eq!(
        requests,
        vec![ClientMessage::FramebufferUpdateRequest {
            incremental: false,
            rect: Rect::new(0, 0, 80, 60),
        }],
        "a refresh asks for the whole desktop at its new size"
    );
}
