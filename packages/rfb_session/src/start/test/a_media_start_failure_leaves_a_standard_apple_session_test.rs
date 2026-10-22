use rfb::{APPLE_PSEUDO_ENCODINGS, ClientMessage};

use crate::test_support::{
    FakeServer, failing_media_starter, high_performance_test_options, next_event,
    read_client_messages, serve_high_performance_mac, start_with_media,
};
use crate::{Quality, SessionEvent, encodings_for_quality};

#[test]
fn test_a_media_start_failure_leaves_a_standard_apple_session() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_mac(stream, 64, 48);
        read_client_messages(stream, 9)
    });
    let (_session, events) = start_with_media(
        high_performance_test_options(server.port()),
        failing_media_starter(),
    );

    let connected = next_event(&events);
    let setup = server.outcome();

    let mut encodings = encodings_for_quality(Quality::Balanced);
    encodings.extend(APPLE_PSEUDO_ENCODINGS);
    assert!(
        matches!(
            connected,
            SessionEvent::Connected {
                apple_remote_desktop: true,
                ..
            }
        ),
        "the session still connects in Apple Remote Desktop mode, got {connected:?}"
    );
    assert!(
        setup.contains(&ClientMessage::SetEncodings(encodings)),
        "without media the client asks for the standard Apple encodings and no virtual display, got {setup:?}"
    );
}
