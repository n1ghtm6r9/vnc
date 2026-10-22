use std::sync::Arc;

use super::GrabbedKey;

#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) struct GrabState {
    pub(crate) window: isize,
    pub(crate) active: bool,
    pub(crate) on_key: Arc<dyn Fn(GrabbedKey) + Send + Sync>,
    pub(crate) swallowed: Vec<u32>,
}
