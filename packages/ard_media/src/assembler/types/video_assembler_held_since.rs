use std::time::Instant;

use super::VideoAssembler;

impl VideoAssembler {
    pub fn held_since(&self) -> Option<Instant> {
        self.pictures.values().map(|picture| picture.first_at).min()
    }
}
