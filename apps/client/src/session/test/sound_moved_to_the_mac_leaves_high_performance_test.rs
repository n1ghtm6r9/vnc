use crate::{SoundChange, SoundPlace, sound_change};

#[test]
fn test_sound_moved_to_the_mac_leaves_high_performance() {
    assert_eq!(
        sound_change(SoundPlace::Mac, true, true),
        SoundChange::LeaveHighPerformance,
        "sound moved to the Mac during High Performance reconnects in standard mode, where the Mac keeps its sound"
    );
    assert_eq!(
        sound_change(SoundPlace::Mac, false, true),
        SoundChange::Keep,
        "a standard session already leaves the sound on the Mac"
    );
}
