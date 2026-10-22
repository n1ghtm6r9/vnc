use rfb::{APPLE_PSEUDO_ENCODINGS, ClientMessage};

use crate::test_support::{
    FakeServer, apple_test_options, next_event, read_client_messages, serve_apple_until_connected,
    start_with_events,
};
use crate::{ClientCommand, Quality, encodings_for_quality};

#[test]
fn test_a_quality_change_in_apple_mode_keeps_the_apple_encodings() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    session.send(ClientCommand::SetEncodings(encodings_for_quality(
        Quality::Fast,
    )));
    let received = server.outcome();

    let mut encodings = encodings_for_quality(Quality::Fast);
    encodings.extend(APPLE_PSEUDO_ENCODINGS);
    assert_eq!(
        received,
        vec![ClientMessage::SetEncodings(encodings)],
        "the new encodings still carry the Apple pseudo-encodings"
    );
}
