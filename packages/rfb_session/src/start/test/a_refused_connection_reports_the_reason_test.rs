use rfb::{ProtocolError, write_security_types, write_text};

use crate::test_support::{
    FakeServer, events_until_disconnected, serve_version, start_with_events, test_options,
};
use crate::{SessionError, SessionEvent};

#[test]
fn test_a_refused_connection_reports_the_reason() {
    let server = FakeServer::start(|stream| {
        serve_version(stream);
        write_security_types(stream, &[]).expect("the fake server refuses the connection");
        write_text(stream, b"too many viewers").expect("the fake server explains the refusal");
    });
    let (_session, events) = start_with_events(test_options(server.port()));

    let events = events_until_disconnected(&events);
    server.outcome();

    assert!(
        matches!(
            events.as_slice(),
            [SessionEvent::Disconnected {
                error: Some(
                    error @ SessionError::Protocol(ProtocolError::ConnectionRefused(reason))
                )
            }] if reason == "too many viewers" && !error.is_authentication_failure()
        ),
        "a refused connection ends with the reason of the server and is no login failure, got {events:?}"
    );
}
