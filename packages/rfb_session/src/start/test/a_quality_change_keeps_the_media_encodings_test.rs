use rfb::{
    APPLE_PSEUDO_ENCODINGS, ClientMessage, ENCODING_APPLE_HIGH_PERFORMANCE,
    ENCODING_APPLE_MEDIA_STREAM,
};

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event,
    read_client_messages, serve_high_performance_until_connected, start_with_media,
};
use crate::{ClientCommand, Quality, encodings_for_quality};

#[test]
fn test_a_quality_change_keeps_the_media_encodings() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        read_client_messages(stream, 1)
    });
    let (start_media, _starts) = fake_media_starter();
    let (session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);

    session.send(ClientCommand::SetEncodings(encodings_for_quality(
        Quality::Fast,
    )));
    let received = server.outcome();

    let mut encodings = vec![ENCODING_APPLE_HIGH_PERFORMANCE];
    encodings.extend(encodings_for_quality(Quality::Fast));
    encodings.extend(APPLE_PSEUDO_ENCODINGS);
    encodings.push(ENCODING_APPLE_MEDIA_STREAM);
    assert_eq!(
        received,
        vec![ClientMessage::SetEncodings(encodings)],
        "a quality change in a High Performance session keeps asking for the media stream"
    );
}
