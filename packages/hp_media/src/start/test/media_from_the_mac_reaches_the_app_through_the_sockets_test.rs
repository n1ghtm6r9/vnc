use crate::{
    MediaChannel, MediaEvent, MediaFactories, MediaState, StripeImage, launch_media,
    test_support::{
        Control, FAKE_FRAME_HEIGHT, FAKE_FRAME_WIDTH, FIRST_DON, NAL_IDR, TEST_PICTURE_HEIGHT,
        TEST_PICTURE_WIDTH, TEST_STRIPES, TestMac, event_channel, fake_frame, fake_unit,
        fixed_secrets, message1_payload, new_fake_decoder, next_event, sound_channel, test_answer,
    },
};

#[test]
fn test_media_from_the_mac_reaches_the_app_through_the_sockets() {
    let mut mac = TestMac::new();
    let (on_event, events) = event_channel();
    let handle = launch_media(
        mac.options(),
        on_event,
        MediaFactories {
            new_decoder: new_fake_decoder,
            new_secrets: fixed_secrets,
            new_sound: sound_channel().0,
        },
    )
    .expect("the media starts on free loopback ports");

    handle.message1(message1_payload());
    next_event(&events, |event| matches!(event, MediaEvent::Offer(_)));
    handle.answer(test_answer());
    let datagrams = mac.server.picture(FIRST_DON, 0, &[&fake_unit(NAL_IDR, 9)]);
    mac.send(MediaChannel::Video, &datagrams, &handle);

    let state = next_event(&events, |event| matches!(event, MediaEvent::State(_)));
    assert_eq!(
        state,
        MediaEvent::State(MediaState::Streaming {
            width: TEST_PICTURE_WIDTH,
            height: TEST_PICTURE_HEIGHT,
            stripes: TEST_STRIPES,
        }),
        "the app hears that High Performance is streaming"
    );
    let stripe = next_event(&events, |event| matches!(event, MediaEvent::Stripe(_)));
    assert_eq!(
        stripe,
        MediaEvent::Stripe(StripeImage {
            y: 0,
            width: FAKE_FRAME_WIDTH,
            height: FAKE_FRAME_HEIGHT,
            frame: fake_frame(FAKE_FRAME_WIDTH, FAKE_FRAME_HEIGHT, 9),
        }),
        "the picture sent to the media port reaches the app as a stripe"
    );
    assert_eq!(
        mac.next_control(MediaChannel::Video, |control| matches!(
            control,
            Control::Ack { .. }
        )),
        Control::Ack { don: FIRST_DON },
        "the picture is acknowledged to the Mac's video port"
    );
}
