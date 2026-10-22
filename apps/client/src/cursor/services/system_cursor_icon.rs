use rfb_session::SystemCursor;
use winit::window::CursorIcon;

pub fn system_cursor_icon(cursor: SystemCursor) -> CursorIcon {
    match cursor {
        SystemCursor::Default => CursorIcon::Default,
        SystemCursor::Text => CursorIcon::Text,
        SystemCursor::Crosshair => CursorIcon::Crosshair,
        SystemCursor::Grabbing => CursorIcon::Grabbing,
        SystemCursor::Grab => CursorIcon::Grab,
        SystemCursor::Pointer => CursorIcon::Pointer,
        SystemCursor::Wait => CursorIcon::Wait,
        SystemCursor::Help => CursorIcon::Help,
        SystemCursor::NotAllowed => CursorIcon::NotAllowed,
        SystemCursor::Move => CursorIcon::Move,
        SystemCursor::NsResize => CursorIcon::NsResize,
        SystemCursor::EwResize => CursorIcon::EwResize,
    }
}
