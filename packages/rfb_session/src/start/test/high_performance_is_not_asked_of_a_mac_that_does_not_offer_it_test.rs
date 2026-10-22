use rfb::{APPLE_PSEUDO_ENCODINGS, ClientMessage};

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event,
    serve_apple_until_connected, start_with_media,
};
use crate::{Quality, encodings_for_quality};

#[test]
fn test_high_performance_is_not_asked_of_a_mac_that_does_not_offer_it() {
    let server = FakeServer::start(|stream| serve_apple_until_connected(stream, 64, 48).1);
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);

    next_event(&events);
    let setup = server.outcome();

    let mut encodings = encodings_for_quality(Quality::Balanced);
    encodings.extend(APPLE_PSEUDO_ENCODINGS);
    assert!(
        setup.contains(&ClientMessage::SetEncodings(encodings)),
        "a Mac without the High Performance capability gets the standard Apple setup, got {setup:?}"
    );
    assert!(
        starts.try_recv().is_err(),
        "no media is started for such a Mac"
    );
}
