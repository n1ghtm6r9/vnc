use ard_media::MSG_MEDIA_STREAM_NEGOTIATION;
use hp_media::MediaEvent;

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    read_framed_message, serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_the_media_offer_goes_to_the_mac() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        read_framed_message(stream)
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);
    let offer = vec![MSG_MEDIA_STREAM_NEGOTIATION, 0, 0, 3, 7, 8, 9];

    (media.emit)(MediaEvent::Offer(offer.clone()));
    let received = server.outcome();

    assert_eq!(received, offer, "the media offer goes to the Mac as it is");
}
