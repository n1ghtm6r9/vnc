use rfb::{
    ClientMessage, ENCODING_DESKTOP_SIZE, Rect, RectHeader, write_framebuffer_update_header,
};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};

#[test]
fn test_a_desktop_resize_is_reported_and_refetched() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 80, 60),
            encoding: ENCODING_DESKTOP_SIZE,
        }
        .write(stream)
        .expect("the fake server announces a new desktop size");
        read_client_messages(stream, 2)
    });
    let (_session, events) = start_with_events(test_options(server.port()));

    let connected = next_event(&events);
    let resized = next_event(&events);
    let requests = server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert!(
        matches!(
            resized,
            SessionEvent::Resized {
                width: 80,
                height: 60
            }
        ),
        "the new desktop size is reported, got {resized:?}"
    );
    assert_eq!(
        requests,
        vec![
            ClientMessage::FramebufferUpdateRequest {
                incremental: true,
                rect: Rect::new(0, 0, 64, 48),
            },
            ClientMessage::FramebufferUpdateRequest {
                incremental: false,
                rect: Rect::new(0, 0, 80, 60),
            },
        ],
        "the client asks ahead for the old desktop, then for the whole new desktop after the resize"
    );
}
