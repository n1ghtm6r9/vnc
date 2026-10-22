use platform::GrabbedKey;
use rfb_session::SessionEvent;

#[derive(Debug)]
pub enum AppEvent {
    Session { generation: u64, event: SessionEvent },
    GrabbedKey(GrabbedKey),
}
