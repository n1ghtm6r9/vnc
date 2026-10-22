use std::io::Write;

use rfb::{
    APPLE_STATE_CURSOR_HIDDEN, APPLE_STATE_CURSOR_VISIBLE, ENCODING_APPLE_CURSOR_ALPHA, Rect,
    RectHeader, write_apple_state_change, write_framebuffer_update_header,
};
use rfb_decode::SystemCursor;

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, serve_apple_until_connected, start_with_events,
};

#[test]
fn test_apple_cursor_changes_reach_the_app() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        write_framebuffer_update_header(stream, 1).expect("the fake server starts an update");
        RectHeader {
            rect: Rect::new(0, 0, 16, 16),
            encoding: ENCODING_APPLE_CURSOR_ALPHA,
        }
        .write(stream)
        .expect("the fake server announces a cursor");
        stream
            .write_all(&[0, 0, 0, 5, 0, 0, 0, 0])
            .expect("the fake server names the pointing hand");
        write_apple_state_change(stream, APPLE_STATE_CURSOR_HIDDEN)
            .expect("the fake server hides the cursor");
        write_apple_state_change(stream, APPLE_STATE_CURSOR_VISIBLE)
            .expect("the fake server shows the cursor");
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let changes = [
        next_event(&events),
        next_event(&events),
        next_event(&events),
    ];
    server.outcome();

    assert!(
        matches!(
            changes,
            [
                SessionEvent::SystemCursorChanged(SystemCursor::Pointer),
                SessionEvent::CursorVisibilityChanged { visible: false },
                SessionEvent::CursorVisibilityChanged { visible: true },
            ]
        ),
        "the system cursor of the Mac and its hiding and showing reach the app, got {changes:?}"
    );
}
