use crate::{SoundChange, SoundPlace, sound_change};

#[test]
fn test_sound_during_high_performance_is_played_or_muted_here() {
    assert_eq!(
        sound_change(SoundPlace::Here, true, true),
        SoundChange::Play(true),
        "sound brought here during High Performance plays without a reconnect"
    );
    assert_eq!(
        sound_change(SoundPlace::Off, true, true),
        SoundChange::Play(false),
        "sound turned off during High Performance is muted without a reconnect"
    );
}
