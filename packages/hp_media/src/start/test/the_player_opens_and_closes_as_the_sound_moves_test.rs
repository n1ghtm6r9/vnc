use std::sync::mpsc::TryRecvError;

use crate::{
    MediaEvent, MediaFactories, launch_media,
    test_support::{
        SoundNotice, TestMac, event_channel, fixed_secrets, message1_payload, new_fake_decoder,
        next_event, next_sound, sound_channel,
    },
};

#[test]
fn test_the_player_opens_and_closes_as_the_sound_moves() {
    let mac = TestMac::new();
    let (on_event, events) = event_channel();
    let (new_sound, notices) = sound_channel();
    let handle = launch_media(
        mac.options(),
        on_event,
        MediaFactories {
            new_decoder: new_fake_decoder,
            new_secrets: fixed_secrets,
            new_sound,
        },
    )
    .expect("the media starts on free loopback ports");

    handle.message1(message1_payload());
    next_event(&events, |event| matches!(event, MediaEvent::Offer(_)));
    let before = notices.try_recv();
    handle.play_sound(true);
    handle.play_sound(true);
    handle.play_sound(false);
    let moved = [next_sound(&notices), next_sound(&notices)];

    assert_eq!(
        before,
        Err(TryRecvError::Empty),
        "no player opens while the sound stays on the Mac"
    );
    assert_eq!(
        moved,
        [SoundNotice::Opened, SoundNotice::Closed],
        "one player opens when the sound comes here and closes when it goes back to the Mac"
    );
}
