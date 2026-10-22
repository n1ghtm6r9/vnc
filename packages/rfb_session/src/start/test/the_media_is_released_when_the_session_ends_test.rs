use std::sync::mpsc::RecvTimeoutError;

use crate::test_support::{
    EVENT_TIMEOUT, FakeServer, events_until_disconnected, fake_media_starter,
    high_performance_test_options, next_event, next_media_start,
    serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_the_media_is_released_when_the_session_ends() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
    });
    let (start_media, starts) = fake_media_starter();
    let (session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let media = next_media_start(&starts);

    drop(session);
    events_until_disconnected(&events);
    let call = media.calls.recv_timeout(EVENT_TIMEOUT);

    assert!(
        matches!(call, Err(RecvTimeoutError::Disconnected)),
        "the session lets go of the media when it ends, which frees its ports, got {call:?}"
    );
}
