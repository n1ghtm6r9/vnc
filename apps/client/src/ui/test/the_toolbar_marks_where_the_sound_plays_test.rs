use crate::{SoundPlace, shown_sound_place};

#[test]
fn test_the_toolbar_marks_where_the_sound_plays() {
    for chosen in [SoundPlace::Here, SoundPlace::Off] {
        assert_eq!(
            shown_sound_place(chosen, true),
            chosen,
            "High Performance plays the sound as chosen: {chosen:?}"
        );
    }
    for chosen in [SoundPlace::Here, SoundPlace::Mac, SoundPlace::Off] {
        assert_eq!(
            shown_sound_place(chosen, false),
            SoundPlace::Mac,
            "without High Performance the Mac plays its own sound, whatever was chosen: {chosen:?}"
        );
    }
}
