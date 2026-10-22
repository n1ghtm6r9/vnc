use super::MvsTileCache;

impl MvsTileCache {
    pub(crate) fn rendered(&self, slot: usize, generation: u32) -> Option<[u32; 64]> {
        if self.rendered_generation.get(slot) != Some(&generation) {
            return None;
        }
        self.rendered.get(slot).copied()
    }
}
