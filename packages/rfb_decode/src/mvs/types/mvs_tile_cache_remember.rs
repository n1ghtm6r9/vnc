use super::MvsTileCache;
use crate::MVS_CACHE_SLOTS;

impl MvsTileCache {
    pub(crate) fn remember(&mut self, slot: usize, generation: u32, pixels: [u32; 64]) {
        if self.rendered.is_empty() {
            self.rendered = vec![[0; 64]; MVS_CACHE_SLOTS];
            self.rendered_generation = vec![0; MVS_CACHE_SLOTS];
        }
        if let (Some(kept), Some(kept_generation)) = (
            self.rendered.get_mut(slot),
            self.rendered_generation.get_mut(slot),
        ) {
            *kept = pixels;
            *kept_generation = generation;
        }
    }
}
