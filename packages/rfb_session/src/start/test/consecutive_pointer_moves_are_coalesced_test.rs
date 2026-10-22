use std::sync::mpsc;

use rfb::ClientMessage;

use crate::ClientCommand;
use crate::test_support::{
    EVENT_TIMEOUT, FakeServer, read_client_messages, serve_apple_login, serve_init, serve_version,
    start_with_events, test_credentials, test_options,
};

#[test]
fn test_consecutive_pointer_moves_are_coalesced() {
    let (open_gate, gate) = mpsc::channel();
    let server = FakeServer::start(move |stream| {
        serve_version(stream);
        serve_apple_login(stream, &test_credentials());
        gate.recv_timeout(EVENT_TIMEOUT)
            .expect("the test queues its commands");
        serve_init(stream, 64, 48);
        read_client_messages(stream, 3);
        read_client_messages(stream, 2)
    });
    let (session, _events) = start_with_events(test_options(server.port()));

    for step in 1..=5 {
        session.send(ClientCommand::Pointer {
            x: step,
            y: step,
            buttons: 0,
        });
    }
    session.send(ClientCommand::Key {
        keysym: 0x61,
        down: true,
    });
    let _ = open_gate.send(());
    let received = server.outcome();

    assert_eq!(
        received,
        vec![
            ClientMessage::PointerEvent {
                buttons: 0,
                x: 5,
                y: 5
            },
            ClientMessage::KeyEvent {
                down: true,
                keysym: 0x61
            },
        ],
        "moves queued while the session was busy reach the server as the last position only"
    );
}
