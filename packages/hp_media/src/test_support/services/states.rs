use crate::{MediaEvent, MediaState};

pub fn states(events: &[MediaEvent]) -> Vec<MediaState> {
    events
        .iter()
        .filter_map(|event| match event {
            MediaEvent::State(state) => Some(state.clone()),
            _ => None,
        })
        .collect()
}
