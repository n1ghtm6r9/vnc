use super::MvsTileCache;
use crate::{MVS_CACHE_ENTRY_BYTES, MVS_CACHE_SLOTS};

impl MvsTileCache {
    pub(crate) fn store(&mut self, entry: &[i8; MVS_CACHE_ENTRY_BYTES]) {
        self.newest += 1;
        if self.newest == MVS_CACHE_SLOTS {
            self.newest = 1;
        }
        self.stored = self.stored.saturating_add(1);
        if self.entries.is_empty() {
            self.entries = vec![[0; MVS_CACHE_ENTRY_BYTES]; MVS_CACHE_SLOTS];
        }
        self.entries[self.newest] = *entry;
        if let Some(generation) = self.rendered_generation.get_mut(self.newest) {
            *generation = 0;
        }
    }
}
