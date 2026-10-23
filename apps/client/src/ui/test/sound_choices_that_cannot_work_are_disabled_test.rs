use crate::{SoundPlace, sound_place_available};

#[test]
fn test_sound_choices_that_cannot_work_are_disabled() {
    assert!(
        sound_place_available(SoundPlace::Mac, SoundPlace::Here, false, false),
        "the Mac can always keep its sound"
    );
    assert!(
        sound_place_available(SoundPlace::Here, SoundPlace::Off, true, true),
        "High Performance can play the sound here"
    );
    assert!(
        sound_place_available(SoundPlace::Here, SoundPlace::Mac, false, true),
        "a session kept standard for the Mac's sound can go back to High Performance"
    );
    assert!(
        !sound_place_available(SoundPlace::Here, SoundPlace::Mac, false, false),
        "the sound cannot come here while the form leaves High Performance off"
    );
    assert!(
        !sound_place_available(SoundPlace::Off, SoundPlace::Here, false, true),
        "a standard session where High Performance did not start cannot mute the Mac"
    );
}
