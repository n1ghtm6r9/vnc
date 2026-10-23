use crate::SoundPlace;

pub fn sound_place_available(
    place: SoundPlace,
    chosen: SoundPlace,
    streamed: bool,
    high_performance_allowed: bool,
) -> bool {
    place == SoundPlace::Mac || streamed || (chosen == SoundPlace::Mac && high_performance_allowed)
}
