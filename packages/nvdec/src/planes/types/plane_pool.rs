use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub(crate) struct PlanePool {
    pub(crate) spares: Arc<Mutex<Vec<Vec<u8>>>>,
}
