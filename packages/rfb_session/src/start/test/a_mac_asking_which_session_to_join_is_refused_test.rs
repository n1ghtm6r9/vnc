use rfb::APPLE_SERVER_FLAG_SESSION_SELECT;

use crate::test_support::{
    FakeServer, MAC_SERVER_FLAGS, apple_test_options, events_until_disconnected, serve_apple_init,
    serve_apple_login, serve_version, start_with_events, test_credentials,
};
use crate::{SessionError, SessionEvent};

#[test]
fn test_a_mac_asking_which_session_to_join_is_refused() {
    let server = FakeServer::start(|stream| {
        serve_version(stream);
        serve_apple_login(stream, &test_credentials());
        serve_apple_init(
            stream,
            64,
            48,
            MAC_SERVER_FLAGS | APPLE_SERVER_FLAG_SESSION_SELECT,
        );
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));

    let events = events_until_disconnected(&events);

    assert!(
        matches!(
            events.as_slice(),
            [SessionEvent::Disconnected {
                error: Some(SessionError::AppleSessionSelect)
            }]
        ),
        "a Mac that wants the client to pick a user session ends the session with a clear reason, got {events:?}"
    );
}
