use std::sync::{Arc, Mutex};

use super::EventCallback;

#[derive(Clone)]
pub(crate) struct EventSink {
    pub(super) target: Arc<Mutex<Option<EventCallback>>>,
}
