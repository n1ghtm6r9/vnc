use rfb::{APPLE_STATE_LOCAL_USER_CLOSED, write_apple_state_change};

use crate::test_support::{
    FakeServer, apple_test_options, events_until_disconnected, serve_apple_until_connected,
    start_with_events,
};
use crate::{SessionError, SessionEvent};

#[test]
fn test_the_mac_user_ending_the_session_is_reported() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_apple_state_change(stream, APPLE_STATE_LOCAL_USER_CLOSED)
            .expect("the fake server reports that its user ended the session");
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));

    let events = events_until_disconnected(&events);
    server.outcome();

    assert!(
        matches!(
            events.as_slice(),
            [
                SessionEvent::Connected { .. },
                SessionEvent::Disconnected {
                    error: Some(SessionError::ClosedOnTheMac)
                }
            ]
        ),
        "the session ends with the reason that the Mac user ended it, got {events:?}"
    );
}
