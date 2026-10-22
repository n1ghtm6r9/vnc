use hp_media::{ChromaLayout, ColourTransform, DecodedFrame, MediaEvent, PlaneBuffer, StripeImage};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, fake_media_starter, high_performance_test_options, next_event, next_media_start,
    serve_high_performance_until_connected, start_with_media,
};

#[test]
fn test_media_stripes_reach_the_app_as_picture_stripes() {
    let server = FakeServer::start(|stream| {
        serve_high_performance_until_connected(stream, 64, 48);
    });
    let (start_media, starts) = fake_media_starter();
    let (_session, events) =
        start_with_media(high_performance_test_options(server.port()), start_media);
    next_event(&events);
    let mut media = next_media_start(&starts);
    let stripe = StripeImage {
        y: 16,
        width: 4,
        height: 2,
        frame: DecodedFrame {
            width: 4,
            height: 2,
            layout: ChromaLayout::Nv12,
            colour: ColourTransform {
                rows: [[1.0, 0.0, 0.0, 0.0]; 3],
                primaries: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            },
            planes: PlaneBuffer::from((0..12).collect::<Vec<u8>>()),
        },
    };

    (media.emit)(MediaEvent::Stripe(stripe.clone()));
    let event = next_event(&events);

    assert!(
        matches!(&event, SessionEvent::PictureStripe(image) if *image == stripe),
        "a stripe of the picture reaches the app with its place and its decoded planes, got {event:?}"
    );
}
