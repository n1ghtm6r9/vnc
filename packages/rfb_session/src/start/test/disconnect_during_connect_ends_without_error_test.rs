use std::sync::mpsc;

use crate::SessionEvent;
use crate::test_support::{
    EVENT_TIMEOUT, FakeServer, drain_until_closed, events_until_disconnected, start_with_events,
    test_options,
};

#[test]
fn test_disconnect_during_connect_ends_without_error() {
    let (accepted, acceptance) = mpsc::channel();
    let server = FakeServer::start(move |stream| {
        let _ = accepted.send(());
        drain_until_closed(stream)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    acceptance
        .recv_timeout(EVENT_TIMEOUT)
        .expect("the fake server accepts the client");

    session.disconnect();
    let events = events_until_disconnected(&events);
    let closed = server.outcome();

    assert!(
        matches!(
            events.as_slice(),
            [SessionEvent::Disconnected { error: None }]
        ),
        "a session stopped while it waits for the server ends without an error, got {events:?}"
    );
    assert!(
        closed,
        "the client closes the connection once it is stopped"
    );
}
