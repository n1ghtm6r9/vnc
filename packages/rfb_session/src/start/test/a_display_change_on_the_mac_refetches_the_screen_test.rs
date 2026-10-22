use std::io::Write;

use rfb::{ClientMessage, Rect};

use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_a_display_change_on_the_mac_refetches_the_screen() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        stream
            .write_all(&[0x51, 0, 0])
            .expect("the fake server reports a display change");
        read_client_messages(stream, 2)
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let received = server.outcome();

    let screen = Rect::new(0, 0, 64, 48);
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
        "after a display change the client asks for the whole screen and for pushed updates again"
    );
}
