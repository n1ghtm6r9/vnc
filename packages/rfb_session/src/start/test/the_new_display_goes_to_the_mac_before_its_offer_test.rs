use ard_media::{MSG_MEDIA_STREAM_NEGOTIATION, virtual_display_for, virtual_display_message};
use hp_media::MediaEvent;

use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    read_framed_message, serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_the_new_display_goes_to_the_mac_before_its_offer() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
        [read_framed_message(stream), read_framed_message(stream)]
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);
    let display = virtual_display_message(&[virtual_display_for(1600, 900, 2)]);
    let offer = vec![MSG_MEDIA_STREAM_NEGOTIATION, 0, 0, 3, 7, 8, 9];

    (media.emit)(MediaEvent::Display(display.clone()));
    (media.emit)(MediaEvent::Offer(offer.clone()));
    let received = server.outcome();

    assert_eq!(
        received,
        [display, offer],
        "the new display reaches the Mac first and the offer for its stream right after"
    );
}
