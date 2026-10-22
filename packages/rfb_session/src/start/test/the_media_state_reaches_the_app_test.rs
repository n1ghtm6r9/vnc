use hp_media::{MediaEvent, MediaState};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_the_media_state_reaches_the_app() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);

    (media.emit)(MediaEvent::State(MediaState::Streaming {
        width: 1280,
        height: 720,
        stripes: 4,
    }));
    let state = next_event(&events);

    assert!(
        matches!(
            state,
            SessionEvent::HighPerformance(MediaState::Streaming {
                width: 1280,
                height: 720,
                stripes: 4
            })
        ),
        "the app learns that the media streams and at what size, got {state:?}"
    );
}
