use crate::{SoundChange, SoundPlace};

pub fn sound_change(place: SoundPlace, high_performance_session: bool, high_performance_allowed: bool) -> SoundChange {
    match (place, high_performance_session) {
        (SoundPlace::Mac, true) => SoundChange::LeaveHighPerformance,
        (SoundPlace::Mac, false) => SoundChange::Keep,
        (_, true) => SoundChange::Play(place == SoundPlace::Here),
        (_, false) if high_performance_allowed => SoundChange::EnterHighPerformance,
        (_, false) => SoundChange::Keep,
    }
}
