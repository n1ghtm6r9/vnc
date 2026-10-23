use crate::{SoundChange, SoundPlace, sound_change};

#[test]
fn test_sound_brought_here_returns_to_high_performance() {
    for place in [SoundPlace::Here, SoundPlace::Off] {
        assert_eq!(
            sound_change(place, false, true),
            SoundChange::EnterHighPerformance,
            "{place:?} chosen in a standard session reconnects in High Performance, the mode that carries the sound"
        );
        assert_eq!(
            sound_change(place, false, false),
            SoundChange::Keep,
            "{place:?} does not turn on High Performance that the form leaves off"
        );
    }
}
