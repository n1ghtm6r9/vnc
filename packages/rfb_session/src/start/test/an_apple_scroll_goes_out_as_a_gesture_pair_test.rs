use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};

#[test]
fn test_an_apple_scroll_goes_out_as_a_gesture_pair() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        read_client_messages(stream, 2)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::Scroll {
        x: 10,
        y: 20,
        right: 0,
        down: 90,
    });
    let received = server.outcome();

    assert!(
        matches!(
            received.as_slice(),
            [
                ClientMessage::AppleGesture {
                    version: 2,
                    kind: 8,
                    ..
                },
                ClientMessage::AppleGesture {
                    version: 1,
                    kind: 11,
                    ..
                },
            ]
        ),
        "a scroll reaches the Mac as a compact scroll and a scroll wheel gesture, got {received:?}"
    );
}
