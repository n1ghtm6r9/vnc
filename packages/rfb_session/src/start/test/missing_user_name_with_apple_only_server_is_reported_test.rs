use std::io::Read;

use rfb::{SECURITY_APPLE_REMOTE_DESKTOP, write_security_types};
use rfb_auth::Credentials;

use crate::test_support::{
    FakeServer, events_until_disconnected, serve_version, start_with_events, test_options,
};
use crate::{ConnectOptions, SessionError, SessionEvent};

#[test]
fn test_missing_user_name_with_apple_only_server_is_reported() {
    let server = FakeServer::start(|stream| {
        serve_version(stream);
        write_security_types(stream, &[SECURITY_APPLE_REMOTE_DESKTOP])
            .expect("the fake server offers only Apple authentication");
        let mut sent = Vec::new();
        let _ = stream.read_to_end(&mut sent);
        sent
    });
    let options = ConnectOptions {
        credentials: Credentials::new("", "secret"),
        ..test_options(server.port())
    };
    let (_session, events) = start_with_events(options);

    let events = events_until_disconnected(&events);
    let sent = server.outcome();

    assert!(
        matches!(
            events.as_slice(),
            [SessionEvent::Disconnected {
                error: Some(error @ SessionError::MissingCredentials)
            }] if error.is_authentication_failure()
        ),
        "an Apple-only server and an empty user name end with missing credentials, got {events:?}"
    );
    assert!(
        sent.is_empty(),
        "the client hangs up without picking a security type, it sent {sent:?}"
    );
}
