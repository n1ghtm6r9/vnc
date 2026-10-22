use crate::{AssemblerEvent, VideoPicture};

pub fn emitted_pictures(events: &[AssemblerEvent]) -> Vec<&VideoPicture> {
    events
        .iter()
        .filter_map(|event| match event {
            AssemblerEvent::Picture(picture) => Some(picture),
            AssemblerEvent::Stalled { .. } => None,
        })
        .collect()
}
