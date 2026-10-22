use std::time::Instant;

use hp_audio::SoundPacket;

use crate::{
    MediaChannel,
    test_support::{FIRST_SEQ, TEST_AUDIO_SSRC, deliver, streaming_session},
};

#[test]
fn test_sound_from_the_mac_is_passed_on() {
    let start = Instant::now();
    let (mut session, mut server) = streaming_session(start);

    let sound = vec![server.audio(), server.audio()];
    deliver(&mut session, MediaChannel::Audio, sound, start);
    let sounds = session.effects.take().sounds;

    let packet = |seq| SoundPacket {
        ssrc: TEST_AUDIO_SSRC,
        seq,
        payload: vec![0; 4],
    };
    assert_eq!(
        sounds,
        vec![packet(FIRST_SEQ), packet(FIRST_SEQ + 1)],
        "each sound packet from the Mac goes on to the player with its place in the sequence"
    );
}
