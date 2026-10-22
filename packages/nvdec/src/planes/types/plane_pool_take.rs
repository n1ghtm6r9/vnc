use super::{PlaneBuffer, PlanePool};

impl PlanePool {
    pub(crate) fn take(&self, len: usize) -> PlaneBuffer {
        let spare = self.spares.lock().ok().and_then(|mut spares| spares.pop());
        let mut bytes = spare.unwrap_or_default();
        bytes.resize(len, 0);
        PlaneBuffer {
            bytes,
            pool: Some(self.clone()),
        }
    }
}
