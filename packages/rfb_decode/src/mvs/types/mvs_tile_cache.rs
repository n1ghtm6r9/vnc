use crate::MVS_CACHE_ENTRY_BYTES;

#[derive(Default)]
pub(crate) struct MvsTileCache {
    pub(super) entries: Vec<[i8; MVS_CACHE_ENTRY_BYTES]>,
    pub(super) rendered: Vec<[u32; 64]>,
    pub(super) rendered_generation: Vec<u32>,
    pub(super) newest: usize,
    pub(super) last_loaded: usize,
    pub(super) stored: usize,
}
