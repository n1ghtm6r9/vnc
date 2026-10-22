use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};

#[test]
fn test_a_display_resize_outside_high_performance_is_dropped() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::ResizeDisplay {
        width: 1920,
        height: 1200,
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
        "without High Performance there is no virtual display to resize, so the key still goes out alone"
    );
}
