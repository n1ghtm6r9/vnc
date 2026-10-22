use rfb::ClientMessage;

use crate::test_support::{
    FakeServer, next_event, read_client_messages, serve_until_connected, start_with_events,
    test_options,
};
use crate::{ClientCommand, Quality, SessionEvent, encodings_for_quality};

#[test]
fn test_a_quality_change_sends_the_new_encodings() {
    let server = FakeServer::start(|stream| {
        serve_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(test_options(server.port()));
    let connected = next_event(&events);

    session.send(ClientCommand::SetEncodings(encodings_for_quality(
        Quality::Fast,
    )));
    let received = server.outcome();

    assert!(
        matches!(connected, SessionEvent::Connected { .. }),
        "the session connects first, got {connected:?}"
    );
    assert_eq!(
        received,
        vec![ClientMessage::SetEncodings(encodings_for_quality(
            Quality::Fast
        ))],
        "a quality change reaches the server as its list of encodings"
    );
}
