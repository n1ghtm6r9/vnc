use crate::{Settings, SoundPlace};

#[test]
fn test_the_sound_plays_here_by_default() {
    assert_eq!(
        Settings::default().sound,
        SoundPlace::Here,
        "a fresh install and an older settings file play the Mac's High Performance sound on this PC"
    );
}
