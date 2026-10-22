use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    serve_high_performance_until_connected, start_with_media,
};
use crate::{ConnectOptions, HighPerformanceOptions};

#[test]
fn test_sound_kept_on_the_mac_is_not_played_by_the_media() {
    let server =
        FakeServer::start(|stream| serve_high_performance_until_connected(stream, 64, 48));
    let (start_media, starts) = fake_media_starter();
    let options = high_performance_test_options(server.port());
    let wanted = options.high_performance.map(|wanted| HighPerformanceOptions {
        play_sound: false,
        ..wanted
    });
    let (_session, events) =
        start_with_media(ConnectOptions { high_performance: wanted, ..options }, start_media);

    next_event(&events);
    let media = next_media_start(&starts);
    server.outcome();

    assert!(
        !media.options.play_sound,
        "sound the user keeps on the Mac is not played by the media here"
    );
}
