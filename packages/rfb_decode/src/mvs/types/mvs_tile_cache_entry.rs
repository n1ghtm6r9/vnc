use super::MvsTileCache;
use crate::MVS_CACHE_ENTRY_BYTES;

impl MvsTileCache {
    pub(crate) fn entry(&self, slot: usize) -> Option<[i8; MVS_CACHE_ENTRY_BYTES]> {
        if slot == 0 || slot > self.stored {
            return None;
        }
        self.entries.get(slot).copied()
    }
}
