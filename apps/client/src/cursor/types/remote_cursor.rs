use winit::window::{CursorIcon, CustomCursor};

#[derive(Debug, Clone)]
pub enum RemoteCursor {
    Hidden,
    Shown(CustomCursor),
    System(CursorIcon),
}
