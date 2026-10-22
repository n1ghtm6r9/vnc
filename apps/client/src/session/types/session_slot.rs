use rfb_session::SessionHandle;

#[derive(Debug, Default)]
pub struct SessionSlot {
    pub(super) generation: u64,
    pub(super) handle: Option<SessionHandle>,
}
