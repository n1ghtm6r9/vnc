use std::time::Instant;

use crate::{
    MediaChannel, MediaState, StripeImage,
    test_support::{
        Control, FAKE_FRAME_HEIGHT, FAKE_FRAME_WIDTH, FIRST_DON, FakeDecoder, NAL_IDR,
        TEST_PICTURE_HEIGHT, TEST_PICTURE_WIDTH, TEST_STRIPES, TestServer, answered_session,
        deliver, fake_frame, fake_unit, states, stripe_images,
    },
};

#[test]
fn test_the_first_decoded_picture_starts_streaming() {
    let start = Instant::now();
    let mut session = answered_session(FakeDecoder::default(), start);
    let mut server = TestServer::new();

    deliver(
        &mut session,
        MediaChannel::Video,
        server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 7)]),
        start,
    );
    let effects = session.effects.take();

    assert_eq!(
        stripe_images(&effects.events),
        [StripeImage {
            y: 0,
            width: FAKE_FRAME_WIDTH,
            height: FAKE_FRAME_HEIGHT,
            frame: fake_frame(FAKE_FRAME_WIDTH, FAKE_FRAME_HEIGHT, 7),
        }],
        "the picture is decoded into the top stripe"
    );
    assert_eq!(
        states(&effects.events),
        [MediaState::Streaming {
            width: TEST_PICTURE_WIDTH,
            height: TEST_PICTURE_HEIGHT,
            stripes: TEST_STRIPES,
        }],
        "the first picture on screen starts High Performance with the answered size"
    );
    assert_eq!(
        server.open_all(&effects.datagrams),
        [Control::Ack { don: FIRST_DON }],
        "the Mac hears that the picture arrived"
    );
}
