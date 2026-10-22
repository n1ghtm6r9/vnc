use std::{io::ErrorKind, time::Duration};

use rfb::ProtocolError;

use crate::test_support::{
    FakeServer, drain_until_closed, events_until_disconnected, start_with_events, test_options,
};
use crate::{ConnectOptions, SessionError, SessionEvent};

#[test]
fn test_a_server_that_never_answers_ends_with_a_timeout() {
    let server = FakeServer::start(drain_until_closed);
    let options = ConnectOptions {
        connect_timeout: Duration::from_millis(300),
        ..test_options(server.port())
    };
    let (_session, events) = start_with_events(options);

    let events = events_until_disconnected(&events);
    let closed = server.outcome();

    assert!(
        matches!(
            events.as_slice(),
            [SessionEvent::Disconnected {
                error: Some(SessionError::Protocol(ProtocolError::Io(cause)))
            }] if cause.kind() == ErrorKind::TimedOut
        ),
        "a server that accepts and then stays silent ends the session with a timeout, got {events:?}"
    );
    assert!(closed, "the client closes the connection after the timeout");
}
