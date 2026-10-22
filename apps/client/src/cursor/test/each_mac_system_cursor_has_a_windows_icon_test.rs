use rfb_session::SystemCursor;
use winit::window::CursorIcon;

use crate::system_cursor_icon;

#[test]
fn test_each_mac_system_cursor_has_a_windows_icon() {
    let cases = [
        (SystemCursor::Default, CursorIcon::Default),
        (SystemCursor::Text, CursorIcon::Text),
        (SystemCursor::Crosshair, CursorIcon::Crosshair),
        (SystemCursor::Grabbing, CursorIcon::Grabbing),
        (SystemCursor::Grab, CursorIcon::Grab),
        (SystemCursor::Pointer, CursorIcon::Pointer),
        (SystemCursor::Wait, CursorIcon::Wait),
        (SystemCursor::Help, CursorIcon::Help),
        (SystemCursor::NotAllowed, CursorIcon::NotAllowed),
        (SystemCursor::Move, CursorIcon::Move),
        (SystemCursor::NsResize, CursorIcon::NsResize),
        (SystemCursor::EwResize, CursorIcon::EwResize),
    ];
    for (cursor, icon) in cases {
        assert_eq!(system_cursor_icon(cursor), icon, "the {cursor:?} cursor of the Mac shows as the {icon:?} icon of Windows");
    }
}
