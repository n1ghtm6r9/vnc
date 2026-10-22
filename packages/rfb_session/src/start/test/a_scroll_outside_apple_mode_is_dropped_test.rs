use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};

#[test]
fn test_a_scroll_outside_apple_mode_is_dropped() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::Scroll {
        x: 10,
        y: 20,
        right: 0,
        down: 90,
    });
    session.send(ClientCommand::Key {
        keysym: 0x61,
        down: true,
    });
    let received = server.outcome();

    assert_eq!(
        received,
        vec![ClientMessage::KeyEvent {
            down: true,
            keysym: 0x61
        }],
        "plain RFB has no scroll gesture, so the scroll is dropped and the key still goes out"
    );
}
