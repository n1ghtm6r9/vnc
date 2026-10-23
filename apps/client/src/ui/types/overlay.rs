use super::{ReconnectView, ToolbarView};
use crate::Language;

pub enum Overlay {
    Form(Language),
    Toolbar(ToolbarView),
    Reconnect(ReconnectView),
}
