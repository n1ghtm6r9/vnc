use std::sync::atomic::AtomicBool;

#[derive(Debug, Default)]
pub(crate) struct SessionControl {
    pub(super) stopped: AtomicBool,
}
