use super::MvsTileCache;

impl MvsTileCache {
    pub(crate) fn next_slot(&self) -> usize {
        (self.last_loaded + 1) & 0xFFFF
    }
}
