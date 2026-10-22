use crate::{
    MediaChannel, MediaEvent, MediaFactories, MediaOptions, launch_media,
    test_support::{
        SoundNotice, TEST_AUDIO_SSRC, TestMac, event_channel, fixed_secrets, message1_payload,
        new_fake_decoder, next_event, next_sound, sound_channel, test_answer,
    },
};

#[test]
fn test_sound_from_the_mac_reaches_the_player() {
    let mut mac = TestMac::new();
    let (on_event, events) = event_channel();
    let (new_sound, notices) = sound_channel();
    let handle = launch_media(
        MediaOptions {
            play_sound: true,
            ..mac.options()
        },
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
    handle.answer(test_answer());
    let sound = mac.server.audio();
    mac.send(MediaChannel::Audio, &[sound], &handle);

    assert_eq!(
        next_sound(&notices),
        SoundNotice::Opened,
        "the player opens as the media starts"
    );
    let played = next_sound(&notices);
    assert!(
        matches!(&played, SoundNotice::Played(packet) if packet.ssrc == TEST_AUDIO_SSRC),
        "the sound sent to the media port reaches the player, not {played:?}"
    );
}
