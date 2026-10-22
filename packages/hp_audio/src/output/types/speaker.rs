use std::time::Instant;

use super::AudioOutput;

#[derive(Default)]
pub(crate) struct Speaker {
    pub(crate) output: Option<AudioOutput>,
    pub(crate) next_check: Option<Instant>,
    pub(crate) problem: Option<String>,
}
