use crate::SoundPlace;

pub fn shown_sound_place(chosen: SoundPlace, streamed: bool) -> SoundPlace {
    if streamed { chosen } else { SoundPlace::Mac }
}
