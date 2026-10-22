use std::io::Write;

use rfb::{
    ClientMessage, ENCODING_APPLE_DISPLAY_INFO_2, Rect, RectHeader, write_framebuffer_update_header,
};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_an_apple_display_resize_refetches_the_new_screen() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 80, 60),
            encoding: ENCODING_APPLE_DISPLAY_INFO_2,
        }
        .write(stream)
        .expect("the fake server announces new displays");
        stream
            .write_all(&[0, 0])
            .expect("the fake server sends an empty display info");
        read_client_messages(stream, 2)
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let resized = next_event(&events);
    let received = server.outcome();

    assert!(
        matches!(
            resized,
            SessionEvent::Resized {
                width: 80,
                height: 60
            }
        ),
        "the new size of the displays is reported, got {resized:?}"
    );
    let screen = Rect::new(0, 0, 80, 60);
    assert_eq!(
        received,
        vec![
            ClientMessage::FramebufferUpdateRequest {
                incremental: false,
                rect: screen
            },
            ClientMessage::AppleAutoUpdate {
                enabled: true,
                rect: screen
            },
        ],
        "after the resize the client asks for the whole new screen and for pushed updates of it"
    );
}
