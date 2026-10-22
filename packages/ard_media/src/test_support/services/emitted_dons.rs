use super::emitted_pictures;
use crate::AssemblerEvent;

pub fn emitted_dons(events: &[AssemblerEvent]) -> Vec<i64> {
    emitted_pictures(events)
        .iter()
        .map(|picture| picture.don)
        .collect()
}
