use std::io::Write;

use rfb::{
    APPLE_STATE_CURSOR_HIDDEN, APPLE_STATE_CURSOR_VISIBLE, SERVER_BELL, write_apple_state_change,
};

use crate::SessionEvent;
use crate::test_support::{
    FakeServer, apple_test_options, next_event, serve_apple_until_connected, start_with_events,
};

#[test]
fn test_a_repeated_cursor_state_reaches_the_app_once() {
    let server = FakeServer::start(|stream| {
        serve_apple_until_connected(stream, 64, 48);
        for state in [
            APPLE_STATE_CURSOR_VISIBLE,
            APPLE_STATE_CURSOR_VISIBLE,
            APPLE_STATE_CURSOR_HIDDEN,
            APPLE_STATE_CURSOR_HIDDEN,
            APPLE_STATE_CURSOR_VISIBLE,
        ] {
            write_apple_state_change(stream, state)
                .expect("the fake server reports the state of its cursor");
        }
        stream
            .write_all(&[SERVER_BELL])
            .expect("the fake server rings the bell");
    });
    let (_session, events) = start_with_events(apple_test_options(server.port()));
    next_event(&events);

    let changes = [
        next_event(&events),
        next_event(&events),
        next_event(&events),
        next_event(&events),
    ];
    server.outcome();

    assert!(
        matches!(
            changes,
            [
                SessionEvent::CursorVisibilityChanged { visible: true },
                SessionEvent::CursorVisibilityChanged { visible: false },
                SessionEvent::CursorVisibilityChanged { visible: true },
                SessionEvent::Bell,
            ]
        ),
        "the Mac repeats the state of its cursor, and only the changes reach the app, got {changes:?}"
    );
}
