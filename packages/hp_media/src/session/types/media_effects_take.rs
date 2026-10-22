use std::mem;

use super::MediaEffects;

impl MediaEffects {
    pub(crate) fn take(&mut self) -> MediaEffects {
        mem::take(self)
    }
}
