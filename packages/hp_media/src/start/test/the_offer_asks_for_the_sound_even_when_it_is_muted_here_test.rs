use crate::{
    MediaEvent, MediaFactories, MediaOptions, launch_media,
    test_support::{
        TestMac, event_channel, fixed_secrets, message1_payload, new_fake_decoder, next_event,
        sound_channel,
    },
};

#[test]
fn test_the_offer_asks_for_the_sound_even_when_it_is_muted_here() {
    for play_sound in [true, false] {
        let mac = TestMac::new();
        let (on_event, events) = event_channel();
        let handle = launch_media(
            MediaOptions {
                play_sound,
                ..mac.options()
            },
            on_event,
            MediaFactories {
                new_decoder: new_fake_decoder,
                new_secrets: fixed_secrets,
                new_sound: sound_channel().0,
            },
        )
        .expect("the media starts on free loopback ports");

        handle.message1(message1_payload());
        let MediaEvent::Offer(offer) =
            next_event(&events, |event| matches!(event, MediaEvent::Offer(_)))
        else {
            panic!("the media answers the Mac with an offer");
        };

        assert_ne!(
            offer[10..12],
            [0, 0],
            "the offer asks the Mac for its sound even when it is muted here, because the Mac refuses an offer without it"
        );
    }
}
