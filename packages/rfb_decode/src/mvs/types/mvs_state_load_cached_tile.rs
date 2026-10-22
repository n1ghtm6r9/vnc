use super::MvsState;
use crate::{MVS_BLUE, MVS_CACHED_LUMA, MVS_CACHED_RED, MVS_LUMA, MVS_RED};

impl MvsState {
    pub(super) fn load_cached_tile(&mut self, slot: usize) -> Option<[u32; 64]> {
        let Some(entry) = self.cache.entry(slot) else {
            log::warn!(
                "[DECODE] the MVS cache slot {slot} is empty, {} tiles have been cached",
                self.cache.stored
            );
            return None;
        };
        let generation = self.quantization_generation;
        let pixels = match self.cache.rendered(slot, generation) {
            Some(pixels) => pixels,
            None => {
                self.coefficients = [[0; 64]; 3];
                let (luma, chroma) = entry.split_at(MVS_CACHED_LUMA);
                let (red, blue) = chroma.split_at(MVS_CACHED_RED);
                for (plane, cached) in [(MVS_LUMA, luma), (MVS_RED, red), (MVS_BLUE, blue)] {
                    for (coefficient, &value) in self.coefficients[plane].iter_mut().zip(cached) {
                        *coefficient = i16::from(value);
                    }
                }
                let pixels = self.render_tile();
                self.cache.remember(slot, generation, pixels);
                pixels
            }
        };
        self.cache.last_loaded = slot;
        Some(pixels)
    }
}
