use rfb_decode::DecodeError;

use crate::test_support::{
    FakeServer, events_until_disconnected, serve_apple_login, serve_init, serve_version,
    start_with_events, test_credentials, test_options,
};
use crate::{SessionError, SessionEvent};

#[test]
fn test_a_desktop_size_outside_the_supported_range_is_refused() {
    for (width, height) in [(12000, 12000), (0, 48), (64, 0)] {
        let server = FakeServer::start(move |stream| {
            serve_version(stream);
            serve_apple_login(stream, &test_credentials());
            serve_init(stream, width, height);
        });
        let (_session, events) = start_with_events(test_options(server.port()));

        let events = events_until_disconnected(&events);
        server.outcome();

        assert!(
            matches!(
                events.as_slice(),
                [SessionEvent::Disconnected {
                    error: Some(SessionError::Decode(DecodeError::Corrupt(_)))
                }]
            ),
            "a {width}x{height} desktop ends the session before Connected, got {events:?}"
        );
    }
}
