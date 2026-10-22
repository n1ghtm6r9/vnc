use super::PlanePool;
use crate::MAX_SPARE_PLANES;

impl PlanePool {
    pub(crate) fn give_back(&self, bytes: Vec<u8>) {
        if let Ok(mut spares) = self.spares.lock()
            && spares.len() < MAX_SPARE_PLANES
        {
            spares.push(bytes);
        }
    }
}
