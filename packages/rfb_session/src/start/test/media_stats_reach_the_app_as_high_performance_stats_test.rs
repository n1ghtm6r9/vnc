use hp_media::{MediaEvent, MediaStats};
use rfb::ENCODING_APPLE_HIGH_PERFORMANCE;

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    serve_high_performance_until_connected, start_with_media,
};
use crate::{SessionEvent, SessionStats};

#[test]
fn test_media_stats_reach_the_app_as_high_performance_stats() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);

    (media.emit)(MediaEvent::Stats(MediaStats {
        bytes_per_second: 6_000_000,
        frames_per_second: 60.0,
    }));
    let stats = next_event(&events);

    assert!(
        matches!(
            stats,
            SessionEvent::Stats(SessionStats {
                bytes_per_second: 6_000_000,
                updates_per_second,
                last_encoding: ENCODING_APPLE_HIGH_PERFORMANCE,
            }) if (updates_per_second - 60.0).abs() < f32::EPSILON
        ),
        "the media stats reach the app as the stats of encoding 1010, got {stats:?}"
    );
}
