use rfb::ClientMessage;

use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};
use crate::{ClientCommand, SessionEvent};

#[test]
fn test_keys_and_pointer_reach_the_server_in_order() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        read_client_messages(stream, 4)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    let connected = next_event(&events);

    session.send(ClientCommand::Key {
        keysym: 0x61,
        down: true,
    });
    session.send(ClientCommand::Pointer {
        x: 10,
        y: 20,
        buttons: 1,
    });
    session.send(ClientCommand::Key {
        keysym: 0x61,
        down: false,
    });
    session.send(ClientCommand::Pointer {
        x: 10,
        y: 20,
        buttons: 0,
    });
    let received = server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert_eq!(
        received,
        vec![
            ClientMessage::KeyEvent {
                down: true,
                keysym: 0x61
            },
            ClientMessage::PointerEvent {
                buttons: 1,
                x: 10,
                y: 20
            },
            ClientMessage::KeyEvent {
                down: false,
                keysym: 0x61
            },
            ClientMessage::PointerEvent {
                buttons: 0,
                x: 10,
                y: 20
            },
        ],
        "key and pointer events reach the server in the order they were sent"
    );
}
