use crate::SessionEvent;
use crate::test_support::{
    FakeServer, drain_until_closed, events_until_disconnected, next_event, serve_until_connected,
    start_with_events, test_options,
};

#[test]
fn test_dropping_the_handle_ends_the_session() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        drain_until_closed(stream)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    let connected = next_event(&events);

    drop(session);
    let rest = events_until_disconnected(&events);
    let closed = server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert!(
        matches!(
            rest.as_slice(),
            [SessionEvent::Disconnected { error: None }]
        ),
        "dropping the handle ends the session without an error, got {rest:?}"
    );
    assert!(
        closed,
        "the client closes the connection when its handle is dropped"
    );
}
