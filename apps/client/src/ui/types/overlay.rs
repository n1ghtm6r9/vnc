use super::{ReconnectView, ToolbarView};

pub enum Overlay {
    Form,
    Toolbar(ToolbarView),
    Reconnect(ReconnectView),
}
