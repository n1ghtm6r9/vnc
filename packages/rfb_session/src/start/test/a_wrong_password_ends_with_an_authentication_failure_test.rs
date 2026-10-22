use rfb::ProtocolError;
use rfb_auth::Credentials;

use crate::test_support::{
    FakeServer, events_until_disconnected, serve_apple_login, serve_version, start_with_events,
    test_credentials, test_options,
};
use crate::{ConnectOptions, SessionError, SessionEvent};

#[test]
fn test_a_wrong_password_ends_with_an_authentication_failure() {
    let server = FakeServer::start(|stream| {
        serve_version(stream);
        serve_apple_login(stream, &test_credentials())
    });
    let options = ConnectOptions {
        credentials: Credentials::new("bogdan", "wrong password"),
        ..test_options(server.port())
    };
    let (_session, events) = start_with_events(options);

    let events = events_until_disconnected(&events);
    let checked = server.outcome();

    assert_eq!(
        checked.credentials.password, "wrong password",
        "the server checks the password the user typed"
    );
    assert!(
        matches!(
            events.as_slice(),
            [SessionEvent::Disconnected {
                error: Some(
                    error @ SessionError::Protocol(ProtocolError::AuthenticationFailed(reason))
                )
            }] if reason == "authentication failed" && error.is_authentication_failure()
        ),
        "a rejected password ends the session at once with an authentication failure, got {events:?}"
    );
}
